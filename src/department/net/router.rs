use tokio::net::{TcpListener, TcpStream};
use log::{debug, error};
use std::convert::Infallible;
//use crate::proto::debugger;
use crossbeam_channel::Receiver;
use crate::department::types::msg;
use tokio::time::{sleep};
use std::time::Duration;



async fn listen_from_render(render_recv: Receiver<msg::TransferMsg>) {
    loop {
        if let Ok(msg) = render_recv.try_recv() {
            
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
                debug!("accept stream from {}", addr);
            }
            Err(e) => {
                error!("error  when accept stream {}", e);
            }
        }
    }
}



