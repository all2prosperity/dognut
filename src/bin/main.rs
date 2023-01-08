use dognut::department::net::router;
use dognut::department::view::window;
use dognut::department::view::render;
use dognut::wgpu::wgpu_helper;
use std::thread;


use crossbeam_channel::unbounded;


fn main () {
    //env_logger::init();
    println!("hello");
    dognut::department::common::logger::App::trivial_conf();

    let (render_pc_s, render_pc_r) = unbounded();
    let (render_cli_s, render_cli_r) = unbounded();

    thread::spawn(|| router::net_run(render_cli_r));
    // thread::spawn(|| render::run(render_pc_s, render_cli_s));
    //thread::spawn(|| wgpu_helper::run(render_pc_s, render_cli_s));

    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();

    rt.block_on(window::run(render_pc_r)).expect("fail on block");
}
