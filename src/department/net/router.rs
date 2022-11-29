use tokio::net::{
    TcpListener, TcpStream,
    tcp::OwnedWriteHalf
};
use log::{debug, error};
use std::convert::Infallible;
//use crate::proto::debugger;
use crossbeam_channel::Receiver;
use crate::department::types::msg;
use tokio::time::{sleep};
use std::time::Duration;
use std::sync::Arc;
use tokio::sync::Mutex;
use lazy_static::lazy_static;
use crate::proto::display;
use prost::Message;


lazy_static! {
    static ref CLIENT_SENDERS: Arc<Mutex<Vec<OwnedWriteHalf>>> = Arc::new(Mutex::new(Vec::new()));
}


async fn listen_from_render(render_recv: Receiver<msg::TransferMsg>) {
    loop {
        if let Ok(msg) = render_recv.try_recv() {
            match msg {
                msg::TransferMsg::RenderPc(frame) => {
                    let mut buf = Vec::<u8>::new();
                    display::Frame {
                        data: frame
                    }.encode_length_delimited(&mut buf);


                    println!("will send to client");
                    for sender in CLIENT_SENDERS.lock().await.iter_mut() {
                        sender.try_write(&buf);
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


pub async fn ws_accept( l: &mut TcpListener, render_recv: Receiver<msg::TransferMsg>) -> Result<(), Infallible>{
    tokio::spawn(listen_from_render(render_recv));

    loop {
        match l.accept().await {
            Ok((stream, addr)) => {
                //tokio::spawn(trans_websocket(stream, addr.to_string()));
                let (client_recv, client_sender) = stream.into_split();
                CLIENT_SENDERS.lock().await.push(client_sender);
                debug!("accept stream from {}", addr);
            }
            Err(e) => {
                error!("error  when accept stream {}", e);
            }
        }
    }
}



