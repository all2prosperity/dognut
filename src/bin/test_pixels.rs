#![deny(clippy::all)]
#![forbid(unsafe_code)]

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use pixels::wgpu::Color;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use dognut::department::preview::homo_transformation::{HomoTransform, Transform};
use dognut::department::view::camera::Camera;
use dognut::department::model::object_buffer::ObjectBuffer;
use dognut::department::preview::matrix::{HMat, Matrix};
use dognut::department::preview::vector::Vector3;
use dognut::department::preview::position::Pos3;
use dognut::department::model::render_object::RenderObject;
use dognut::department::model::object_loader::ObjectLoader;
use dognut::department::model::triangle_resources::TriangleResources;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World {
    camera: Camera,
    objs: Vec<RenderObject>,
    resources: TriangleResources,
    theta: f32,
}

fn main() -> Result<(), Error> {
    env_logger::init();
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
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    pixels.set_clear_color(Color::WHITE);
    let mut world = World::new();

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.get_frame_mut());
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

            if input.key_pressed(VirtualKeyCode::A) {
                world.camera.move_view(VirtualKeyCode::A);
            }
            else if input.key_pressed(VirtualKeyCode::D) {
                world.camera.move_view(VirtualKeyCode::D);
            }
            else if input.key_pressed(VirtualKeyCode::W) {
                world.camera.move_view(VirtualKeyCode::W);
            }
            else if input.key_pressed(VirtualKeyCode::S) {
                world.camera.move_view(VirtualKeyCode::S);
            }
            else if input.key_pressed(VirtualKeyCode::Q) {
                world.camera.move_view(VirtualKeyCode::Q);
            }
            else if input.key_pressed(VirtualKeyCode::E) {
                world.camera.move_view(VirtualKeyCode::E);
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state and request a redraw
            world.update();
            window.request_redraw();
        }
    });
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        let objs = ObjectLoader::load_render_obj("./res/cube.obj");
        for i in &objs {
            println!("i len:{:?}, pos:{:?}", i.indexes.len(), i.vertexes.len());
        }
        let res = ObjectLoader::load_triangle_resources("./res/cube/.obj");

        Self {
            camera: Camera::new(45., (WIDTH / HEIGHT) as f32, -5., -50., Pos3::from_xyz(0., 0., 10.,),
                                Vector3::from_xyz(0., 0., -1.),
                                Vector3::from_xyz(0., -1., 0.)),
            objs: objs,
            resources: res,
            theta: 0.,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&mut self, frame: &mut [u8]) {
        profiling::scope!("Main Thread");
        let mut buffer = ObjectBuffer::new();
        self.theta += 0.02;

        // link_adult is too big
        //let mut scale = HomoTransform::scale((0.01, 0.01, 0.01));
        let mut scale = HomoTransform::identity_matrix();
        scale.mul_num(1.);
        scale.set(3, 3, 1.);

        let _move_origin = HomoTransform::translation((-0., -0., 0.));
        let rotate = Transform::rotation_mat(&Vector3::from_xyz(0.,1.,0.), self.theta);
        // let _move_back = HomoTransform::translation((0., 0., -0.0));
        // let _mat = _move_origin * rotate * _move_back;
        // let _mat = HomoTransform::identity_matrix();
        let _mat = scale * rotate;

        let _move = HMat::from_vec(vec![
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            0., 0., -5.0, 1.,
        ]);

        //let _mat = _mat * scale;
        //let _mat = _mat * _move;

        for i in &self.objs {
            buffer.add_object(i.clone());
        }

        //let _buf = self.camera.render(WIDTH, HEIGHT, &buffer, &_mat);

        let _buf = self.camera.render_triangle_obejct(WIDTH, HEIGHT, &self.resources, &_mat);

        frame.copy_from_slice(&_buf.display);
        profiling::finish_frame!();
    }
}
