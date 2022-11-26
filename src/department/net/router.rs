use tokio::net::{TcpListener, TcpStream};
use log::{debug, error};
use std::convert::Infallible;
//use crate::proto::debugger;


pub async fn ws_accept( l: &mut TcpListener) -> Result<(), Infallible>{
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
