use std::convert::Infallible;
use std::sync::Arc;

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
use crate::department::types::multi_sender::MultiSender;
use crate::department::types::msg::{TransferMsg, DognutOption};


use crate::department::video::encode::RgbaEncoder;
use crate::pb::netpacket::{NetPacket, PacketKind};

lazy_static! {
    static ref CLIENT_SENDERS: Arc<Mutex<Vec<OwnedWriteHalf>>> = Arc::new(Mutex::new(Vec::new()));
}

static mut BIND_PORT: u32 = 0;

pub struct Router {
    client_clicked: bool,
    receiver: Option<Receiver<TransferMsg>>, // for encoder use
    ms: Option<MultiSender<TransferMsg>>,
}

impl Router {
    pub fn new(receiver: Receiver<TransferMsg>, ms: MultiSender<TransferMsg>) -> Self {
        Self { client_clicked: false, receiver: Some(receiver), ms: Some(ms)}
    }

    pub fn run(mut self) {
//        self.start_encoding_thread();
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
            rt.block_on(async {
                for i in 0..(constant::PORT_RANGE) {
                    let host_str = format!("{}:{}", constant::HOST, constant::PORT + i);
                    if let Ok(mut lis) = TcpListener::bind(&host_str).await {
                        println!("Server listen on {}", host_str);
                        unsafe {
                            BIND_PORT = constant::PORT + i;
                        }
                        self.ws_accept(&mut lis).await;
                        break;
                    }
                }
            });
        });
    }

    pub fn start_encoding_and_rendering(&mut self) {
        // RgbaEncoder::run(self.rgba_rx.take().unwrap(), self.pkg_tx.take().unwrap(), (constant::WIDTH, constant::HEIGHT));
        if let Some(_ms) = &mut self.ms {
            //_ms.win.send(TransferMsg::DogOpt(DognutOption::StartRender)).unwrap();
            _ms.enc.send(TransferMsg::DogOpt(DognutOption::StartEncode)).unwrap();
        }
    }

    pub async fn ws_accept(&mut self, l: &mut TcpListener) -> Result<(), Infallible>{
        tokio::spawn(listen_from_render(self.receiver.take().unwrap()));
        tokio::spawn(listen_from_udp());

        loop {
            match l.accept().await {
                Ok((stream, addr)) => {
                    //tokio::spawn(trans_websocket(stream, addr.to_string()));
                    let (_client_recv, client_sender) = stream.into_split();
                    CLIENT_SENDERS.lock().await.push(client_sender);
                    if !self.client_clicked {
                        self.start_encoding_and_rendering();
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
                msg::TransferMsg::RenderedData(frame) => {
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
        sleep(Duration::from_millis(2)).await
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
