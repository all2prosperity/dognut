use log::{LevelFilter};
use dognut::department::net::router;
use dognut::department::view::window;
use dognut::department::types::multi_sender::MultiSender;
use dognut::department::types::msg;
use dognut::department::video::encode::RgbaEncoder;
use dognut::department::common::constant;

fn main () {
    let env = env_logger::Env::default();
    env_logger::Builder::from_env(env).target(env_logger::Target::Stdout).filter(Some("wgpu_core"), LevelFilter::Error).
        filter_level(LevelFilter::Info).init();

    log::info!(target:"wgpu_core", "hello");
    //dognut::department::common::logger::App::trivial_conf();
    //let (rgb_tx, rgb_rx) = crossbeam_channel::bounded::<Vec<u8>>(500);
    let (net_sender, net_receiver) = crossbeam_channel::unbounded::<msg::TransferMsg>();
    let (win_sender, win_receiver) = crossbeam_channel::unbounded::<msg::TransferMsg>();
    let (enc_sender, enc_receiver) = crossbeam_channel::unbounded::<msg::TransferMsg>();

    let ms = MultiSender::new(net_sender, enc_sender, win_sender);

    router::Router::new(net_receiver, ms.clone()).run();
    RgbaEncoder::run(enc_receiver, ms.clone(), (constant::WIDTH, constant::HEIGHT));

    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();

    rt.block_on(window::run(win_receiver, ms)).expect("fail on block");
}
