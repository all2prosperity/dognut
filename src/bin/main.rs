use std::thread;

use crossbeam_channel::unbounded;

use dognut::department::net::router;
use dognut::department::types::multi_sender;
use dognut::department::view::render;
use dognut::department::view::window;
use dognut::wgpu::wgpu_helper;

fn main () {
    env_logger::init();
    println!("hello");
    //dognut::department::common::logger::App::trivial_conf();

    let (net_s, net_r) = unbounded();
    let (wgpu_s, wgpu_r) = unbounded();
    let (win_s, win_r) = unbounded();

    let ms_win = multi_sender::MultiSender::new(net_s, wgpu_s, win_s);
    let ms_net = ms_win.clone();
    let ms_wgpu = ms_win.clone();
    // thread::spawn(|| render::run(render_pc_s, render_cli_s));
    //thread::spawn(|| wgpu_helper::run(render_pc_s, render_cli_s));

    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();

    rt.block_on(window::run(win_r, ms_win)).expect("fail on block");
}
