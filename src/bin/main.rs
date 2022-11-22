use dognut::department::net::router;
use dognut::department::common::constant;
use std::{thread, env};
use tokio::net::{TcpListener};
use log::info;

fn main () {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let host_str = format!("{}:{}", constant::HOST, constant::PORT);
    dognut::department::common::logger::App::trivial_conf();

    rt.block_on(async {
        let addr = env::args()
            .nth(1)
            .unwrap_or_else(|| {host_str});

        let mut lis = TcpListener::bind(&addr).await.expect("can't bind socket");
        info!("Server listen on {}", addr);

        router::ws_accept(&mut lis).await;
    });
}
