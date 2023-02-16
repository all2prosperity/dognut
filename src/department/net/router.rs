use std::convert::Infallible;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

//use crate::proto::debugger;
use crossbeam_channel::{Receiver, unbounded};
use lazy_static::lazy_static;
use log::{debug, error};
use protobuf::Message;
use tokio::io::AsyncWriteExt;
use tokio::net::{
    tcp::OwnedWriteHalf,
    TcpListener,
    UdpSocket
};
use tokio::sync::Mutex;
use tokio::time::sleep;

use crate::department::common::constant;
use crate::department::types::msg;
use crate::department::types::msg::TransferMsg;
use crate::department::types::multi_sender::MultiSender;
use crate::department::video::decode::RgbaDecoder;
use crate::department::video::encode::RgbaEncoder;
use crate::pb::netpacket::{NetPacket, PacketKind};

lazy_static! {
    static ref CLIENT_SENDERS: Arc<Mutex<Vec<OwnedWriteHalf>>> = Arc::new(Mutex::new(Vec::new()));
}

static mut BIND_PORT: u32 = 0;

pub struct Router {
    client_clicked: bool,
    rgba_rx: Option<Receiver<Vec<u8>>>, // for encoder use
    pkg_tx: Option<crossbeam_channel::Sender<TransferMsg>>, // for encoder use
    pkg_rx: Receiver<TransferMsg>,
}

impl Router {
    pub fn new(rgba_rx: Receiver<Vec<u8>>) -> Self {
        let (pkg_tx, pkg_rx) = unbounded();

        Self { client_clicked: false, rgba_rx:Some(rgba_rx), pkg_tx: Some(pkg_tx), pkg_rx }
    }

    pub fn run(mut self) {
        self.start_encoding_thread();
        // thread::spawn(move || {
        //     let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
        //     rt.block_on(async {
        //         for i in 0..(constant::PORT_RANGE) {
        //             let host_str = format!("{}:{}", constant::HOST, constant::PORT + i);
        //             if let Ok(mut lis) = TcpListener::bind(&host_str).await {
        //                 println!("Server listen on {}", host_str);
        //                 unsafe {
        //                     BIND_PORT = constant::PORT + i;
        //                 }
        //                 self.ws_accept(&mut lis).await;
        //                 break;
        //             }
        //         }
        //     });
        // });
    }

    pub fn start_encoding_thread(&mut self) {
        RgbaEncoder::run(self.rgba_rx.take().unwrap(), self.pkg_tx.take().unwrap(), (constant::WIDTH, constant::HEIGHT));
        let fake_channel = crossbeam_channel::unbounded();
        RgbaDecoder::run(self.pkg_rx.clone(), fake_channel.0, (constant::WIDTH, constant::HEIGHT));
    }

    pub async fn ws_accept(&mut self, l: &mut TcpListener) -> Result<(), Infallible>{
        tokio::spawn(listen_from_render(self.pkg_rx.clone()));
        tokio::spawn(listen_from_udp());

        loop {
            match l.accept().await {
                Ok((stream, addr)) => {
                    //tokio::spawn(trans_websocket(stream, addr.to_string()));
                    let (_client_recv, client_sender) = stream.into_split();
                    CLIENT_SENDERS.lock().await.push(client_sender);
                    if !self.client_clicked {
                        self.start_encoding_thread();
                        self.client_clicked = true;
                    }

                    debug!("accept stream from {}", addr);
                }
                Err(e) => {
                    error!("error  when accept stream {}", e);
                }
            }
        }
    }
}

async fn listen_from_render(render_recv: Receiver<msg::TransferMsg>) {
    loop {
        if let Ok(msg) = render_recv.try_recv() {

            match msg {
                msg::TransferMsg::RenderPc(frame) => {
                    let mut net_pkt = NetPacket::new();
                    net_pkt.data = frame;
                    net_pkt.kind = protobuf::EnumOrUnknown::from(PacketKind::VideoPacket);
                    let serialized = net_pkt.write_to_bytes().unwrap();

                    for sender in CLIENT_SENDERS.lock().await.iter_mut() {

                        sender.write_u32( serialized.len() as u32).await.unwrap();
                        sender.write(serialized.as_slice()).await.unwrap();
                    }
                },
                _ => ()
            }

        };
        sleep(Duration::from_millis(100)).await
    }
}



async fn listen_from_udp() {
    let mut buf = [0; 1024];
    for i in 0..(constant::PORT_RANGE) {
        let host_str = format!("{}:{}", constant::HOST, constant::UDP_PORT + i);
        if let Ok(sock) = UdpSocket::bind(host_str.clone()).await {
            println!("listen from udp at:{:?}", host_str);
            loop {
                if let Ok((len, addr)) = sock.recv_from(&mut buf).await {
                    println!("recv buf len:{:?}, addr:{:?}", len, addr);
                    let mut data = json::JsonValue::new_object();
                    unsafe {
                        data["port"] = BIND_PORT.into();
                    }
                    sock.send_to(&data.dump().as_bytes(), addr).await;
                }
                else {
                    println!("recv buf failed:");
                }
            }
        }
    }
}