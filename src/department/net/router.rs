use tokio::net::{
    TcpListener,
    tcp::OwnedWriteHalf,
    UdpSocket
};
use log::{debug, error};
use crate::department::common::constant;
use crate::department::types::msg::TransferMsg;
use std::convert::Infallible;
//use crate::proto::debugger;
use crossbeam_channel::Receiver;
use crate::department::types::msg;

use tokio::time::sleep;
use std::time::Duration;
use std::sync::Arc;
use tokio::sync::Mutex;
use lazy_static::lazy_static;
use prost::bytes;
use prost::bytes::BufMut;
use tokio::io::AsyncWriteExt;
use crate::department::types::multi_sender::MultiSender;




lazy_static! {
    static ref CLIENT_SENDERS: Arc<Mutex<Vec<OwnedWriteHalf>>> = Arc::new(Mutex::new(Vec::new()));
}

static mut BIND_PORT: u32 = 0;


async fn listen_from_render(render_recv: Receiver<msg::TransferMsg>) {
    loop {
        if let Ok(msg) = render_recv.try_recv() {

            match msg {
                msg::TransferMsg::RenderPc(frame) => {
                    let size = frame.len();

                    for sender in CLIENT_SENDERS.lock().await.iter_mut() {
                        sender.write_u32(size as u32).await.unwrap();
                        sender.try_write(&frame).unwrap();
                    }
                },
                _ => ()
            }


            // for sender in client_senders {
            // }
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

pub fn net_run(render_recv: Receiver<TransferMsg>, ms: Option<MultiSender<TransferMsg>>) {
    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    rt.block_on(async {
        for i in 0..(constant::PORT_RANGE) {
            let host_str = format!("{}:{}", constant::HOST, constant::PORT + i);
            if let Ok(mut lis) = TcpListener::bind(&host_str).await {
                println!("Server listen on {}", host_str);
                unsafe {
                    BIND_PORT = constant::PORT + i;
                }
                ws_accept(&mut lis, render_recv).await;
                break;
            }
        }
    });

}

pub async fn ws_accept( l: &mut TcpListener, render_recv: Receiver<msg::TransferMsg>) -> Result<(), Infallible>{
    tokio::spawn(listen_from_render(render_recv));
    tokio::spawn(listen_from_udp());

    loop {
        match l.accept().await {
            Ok((stream, addr)) => {
                //tokio::spawn(trans_websocket(stream, addr.to_string()));
                let (_client_recv, client_sender) = stream.into_split();
                CLIENT_SENDERS.lock().await.push(client_sender);
                debug!("accept stream from {}", addr);
            }
            Err(e) => {
                error!("error  when accept stream {}", e);
            }
        }
    }
}



