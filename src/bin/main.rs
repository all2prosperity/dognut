use log::{error, LevelFilter};
use dognut::department::net::router;
use dognut::department::view::window;
use dognut::department::types::multi_sender::MultiSender;
use dognut::department::types::msg;
#[cfg(feature = "rtc")]
use dognut::department::video::encode::RgbaEncoder;
use dognut::department::common::{constant, self_type};
use dognut::department::common::constant::{HEIGHT, WIDTH};
use dognut::department::pipeline::rasterizer::RasterRunner;
use dognut::department::pipeline::shader::LambertianShader;
use dognut::department::preview::vector::Vector3;
use dognut::department::tui::tui_with_window::TuiWinApp;
use dognut::department::video::ImgEncoder;
use dognut::department::view::camera::Camera;
use dognut::util::ARG;

fn main () {
    let env = env_logger::Env::default();
    env_logger::Builder::from_env(env).target(env_logger::Target::Stdout).filter(Some("wgpu_core"), LevelFilter::Off).
        filter_level(LevelFilter::Info).format_timestamp_millis().init();

    let arg = &ARG;

    log::info!(target:"wgpu_core", "hello");

    let (net_sender, net_receiver) = crossbeam_channel::unbounded::<msg::TransferMsg>();
    let (win_sender, win_receiver) = crossbeam_channel::unbounded::<msg::TransferMsg>();
    let (enc_sender, enc_receiver) = crossbeam_channel::unbounded::<msg::TransferMsg>();

    let ms = MultiSender::new(net_sender, enc_sender, win_sender);

    router::Router::new(net_receiver, ms.clone()).run();
    #[cfg(feature = "rtc")]
    RgbaEncoder::run(enc_receiver, ms.clone(), (WIDTH, HEIGHT));

    #[cfg(not(feature = "rtc"))]
    ImgEncoder::run(enc_receiver, ms.clone(), (WIDTH, HEIGHT));

    if arg.term {
        let tui_ms = ms.clone();
        let raster_ms = ms.clone();

        std::thread::Builder::new().name("tui_renderer_thread".into()).spawn( move || {
            let camera=  Camera::new(45., (WIDTH / HEIGHT) as f32,
                                     -5., -50., Vector3::from_xyz(0., 0., 10.,),
                                     Vector3::from_xyz(0., 0., -1.),
                                     Vector3::from_xyz(0., -1., 0.));
            let shader = LambertianShader::new(Vector3::from_xyz(0., 1., 0.),
                                               0.8, 1.,&camera, arg.term);
            let raster = RasterRunner::new(raster_ms, camera,
                                           Box::new(shader), arg.term);
            let inner_rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();

            inner_rt.block_on(async {
                let res = dognut::department::model::object_loader::ObjectLoader::load_triangle_resources(&arg.obj_path);
                let dimension = (256,79);
                let camera = self_type::camera_instance(WIDTH, HEIGHT);
                let state = dognut::wgpu::wgpu_helper::State::new(winit::dpi::LogicalSize { width: WIDTH, height:HEIGHT }, camera).await;
                let result = TuiWinApp::new(raster,res, tui_ms).run(Some(state));
                if let Err(e) = result {
                    error!("tui return an error, {}", e.to_string());
                };
            });
        }).unwrap();

        dognut::department::view::local_window::start(win_receiver, ms);
        return ;
    }else {
        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
        rt.block_on(window::run(win_receiver, ms)).expect("fail on block");
    }

}
