use std::thread;

use dognut::department::net::router;
use dognut::department::types::multi_sender;
use dognut::department::view::render;
use dognut::department::view::window;
use dognut::wgpu::wgpu_helper;

fn main () {
    env_logger::init();
    println!("hello");
    //dognut::department::common::logger::App::trivial_conf();
    //let (rgb_tx, rgb_rx) = crossbeam_channel::bounded::<Vec<u8>>(500);
    let (rgb_tx, rgb_rx) = crossbeam_channel::unbounded::<Vec<u8>>();

    router::Router::new(rgb_rx).run();

    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();

    rt.block_on(window::run(rgb_tx)).expect("fail on block");
}
