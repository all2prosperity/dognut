

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use pixels::wgpu::Color;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use dognut::department::common::constant;
use dognut::department::common::constant::{HEIGHT, WIDTH};
use dognut::department::common::self_type;
use dognut::department::model::object_loader::ObjectLoader;
use dognut::department::model::triangle_resources::TriangleResources;

use dognut::department::pipeline::rasterizer::RasterRunner;
use dognut::department::pipeline::shader::LambertianShader;
use dognut::department::preview::output_buffer::OutputBuffer;
use dognut::department::preview::vector::Vector3;
use dognut::department::tui::TuiApp;
use dognut::department::types::msg::TransferMsg;
use dognut::department::types::multi_sender::MultiSender;
use dognut::department::video::encode::RgbaEncoder;
use dognut::department::view::camera::Camera;
use dognut::util::{ARG};

use dognut::wgpu::wgpu_helper::State;

fn main() -> Result<(), Error>{
    env_logger::init();
    let arg = &ARG;

    let (net_sender, net_receiver) = crossbeam_channel::unbounded::<TransferMsg>();
    let (win_sender, win_receiver) = crossbeam_channel::unbounded::<TransferMsg>();
    let (enc_sender, enc_receiver) = crossbeam_channel::unbounded::<TransferMsg>();
    let ms = MultiSender::new(net_sender, enc_sender, win_sender);

    let camera=  Camera::new(45., (constant::WIDTH / constant::HEIGHT) as f32,
                             -5., -50., Vector3::from_xyz(0., 0., 10.,),
                             Vector3::from_xyz(0., 0., -1.),
                             Vector3::from_xyz(0., -1., 0.));

    let shader = LambertianShader::new(Vector3::from_xyz(0., 1., 0.),
                                       0.8, 1.,&camera, arg.term);


    let raster = RasterRunner::new(ms.clone(), camera,
                      Box::new(shader), arg.term);

    println!("obj resources path is {}", &arg.obj_path);
    let res = ObjectLoader::load_triangle_resources(&arg.obj_path);

    if arg.term {

        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();

        rt.block_on(async {
            let dimension = (256,79);
            let camera = self_type::camera_instance();
            let handle = RgbaEncoder::run(enc_receiver, ms, (constant::WIDTH, constant::HEIGHT));
            let state = State::new(winit::dpi::LogicalSize { width: dimension.0 as u32, height: dimension.1 as u32 }, camera).await;
            let result = TuiApp::new(raster).run(res, Some(state));
            if let Err(e) = result {
                error!("tui return an error, {}", e.to_string());
            };
            handle.join().unwrap();
        });
        return Ok(());
    }

    if arg.render_a_picture {
        let mut out = OutputBuffer::new(constant::WIDTH, constant::HEIGHT, false);
        raster.render_frame( &res, &mut out);
        out.save_to_image("./img.png");
        return Ok(());
    }

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(constant::WIDTH, constant::HEIGHT, surface_texture)?
    };
    pixels.set_clear_color(Color::WHITE);

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {

            draw(&raster, &res,pixels.get_frame_mut());
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state and request a redraw
            window.request_redraw();
        }
    });

    //handle.join().unwrap();
}

fn draw(raster:&RasterRunner,res: &TriangleResources , frame: &mut [u8]) {
    let mut out = OutputBuffer::new(constant::WIDTH, constant::HEIGHT, false);
    raster.render_frame( res, &mut out);
    frame.copy_from_slice(&out.display);
}
