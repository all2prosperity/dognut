use dognut::department::net::router;
use dognut::department::view::window;
use dognut::department::view::render;
use dognut::department::types::multi_sender;
use dognut::wgpu::wgpu_helper;
use std::thread;


use crossbeam_channel::unbounded;


fn main () {
    //env_logger::init();
    println!("hello");
    dognut::department::common::logger::App::trivial_conf();

    let (net_s, net_r) = unbounded();
    let (wgpu_s, wgpu_r) = unbounded();
    let (win_s, win_r) = unbounded();

    let ms_win = multi_sender::MultiSender::new(net_s, wgpu_s, win_s);
    let ms_net = ms_win.clone();
    let ms_wgpu = ms_win.clone();

    thread::spawn(move || router::net_run(net_r, ms_net));
    // thread::spawn(|| render::run(render_pc_s, render_cli_s));
    thread::spawn(move || wgpu_helper::run(wgpu_r, ms_wgpu));
    window::run(win_r, ms_win);
}
