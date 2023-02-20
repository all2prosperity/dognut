use log::{LevelFilter};
use dognut::department::net::router;
use dognut::department::view::window;


fn main () {
    let env = env_logger::Env::default();
    env_logger::Builder::from_env(env).target(env_logger::Target::Stdout).filter(Some("wgpu_core"), LevelFilter::Error).
        filter_level(LevelFilter::Info).init();

    log::info!(target:"wgpu_core", "hello");
    //dognut::department::common::logger::App::trivial_conf();
    //let (rgb_tx, rgb_rx) = crossbeam_channel::bounded::<Vec<u8>>(500);
    let (rgb_tx, rgb_rx) = crossbeam_channel::unbounded::<Vec<u8>>();

    router::Router::new(rgb_rx).run();

    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();

    rt.block_on(window::run(rgb_tx)).expect("fail on block");
}
