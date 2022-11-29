use dognut::department::net::router;
use dognut::department::common::constant;
use dognut::department::view::window;
use dognut::department::view::render;
use dognut::department::types::msg::TransferMsg;
use std::{thread, env};
use tokio::net::{TcpListener};
use log::info;
use crossbeam_channel::{unbounded, Receiver};


fn net_run(render_recv: Receiver<TransferMsg>) {
    let mut rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    // let mut rt = tokio::runtime::Runtime::new().unwrap();
    let host_str = format!("{}:{}", constant::HOST, constant::PORT);

    rt.block_on(async {
        let addr = env::args()
            .nth(1)
            .unwrap_or_else(|| {host_str});

        let mut lis = TcpListener::bind(&addr).await.expect("can't bind socket");
        info!("Server listen on {}", addr);

        router::ws_accept(&mut lis, render_recv).await;
    });

}


fn main () {
    //env_logger::init();
    println!("hello");
    dognut::department::common::logger::App::trivial_conf();

    let (render_send, render_recv) = unbounded();
    let render_recv2 = render_recv.clone();

    thread::spawn(|| net_run(render_recv2));
    thread::spawn(|| render::run(render_send));
    window::run(render_recv);
}
