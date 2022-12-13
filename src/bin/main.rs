use dognut::department::net::router;
use dognut::department::view::window;
use dognut::department::view::render;

use std::{thread};


use crossbeam_channel::{unbounded};


fn main () {
    //env_logger::init();
    println!("hello");
    dognut::department::common::logger::App::trivial_conf();

    let (render_pc_s, render_pc_r) = unbounded();
    let (render_cli_s, render_cli_r) = unbounded();

    thread::spawn(|| router::net_run(render_cli_r));
    thread::spawn(|| render::run(render_pc_s, render_cli_s));
    window::run(render_pc_r);
}
