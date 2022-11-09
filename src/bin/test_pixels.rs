#![deny(clippy::all)]
#![forbid(unsafe_code)]

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use dognut::department::view::camera::Camera;
use dognut::department::preview::object_buffer::ObjectBuffer;
use dognut::department::preview::matrix::Matrix;
use dognut::department::preview::vector::Vector3;
use dognut::department::preview::render_object::RenderObject;
use dognut::department::preview::object_loader::ObjectLoader;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const BOX_SIZE: i16 = 64;

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World {
    box_x: i16,
    box_y: i16,
    velocity_x: i16,
    velocity_y: i16,
    camera: Camera,
    obj: RenderObject,
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
        let objs = ObjectLoader::load_render_obj("G:\\sh3dMod\\FishSoup_Pot.obj");
        for i in &objs {
            println!("i len:{:?}, pos:{:?}", i.indexes.len(), i.vertexes.len());
        }

        // let obj = RenderObject::from_vec(
        //     vec![
        //         Pos3::new(-0.5, -0.5, -7.),
        //         Pos3::new(0.5, -0.5, -7.),
        //         Pos3::new(0.5, -0.5, -8.),
        //         Pos3::new(-0.5, -0.5, -8.),
        //         Pos3::new(-0.5, 0.5, -7.),
        //         Pos3::new(0.5, 0.5, -7.),
        //         Pos3::new(0.5, 0.5, -8.),
        //         Pos3::new(-0.5, 0.5, -8.),
        //     ],
        //     vec![0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4],
        //     );
        let obj = &objs[1];
        // println!("obj poses:{:?}", obj.vertexes);

        Self {
            box_x: 24,
            box_y: 16,
            velocity_x: 1,
            velocity_y: 1,
            camera: Camera::new(10., 1., -5., -10.),
            obj: obj.clone(),
            theta: 0.,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        if self.box_x <= 0 || self.box_x + BOX_SIZE > WIDTH as i16 {
            self.velocity_x *= -1;
        }
        if self.box_y <= 0 || self.box_y + BOX_SIZE > HEIGHT as i16 {
            self.velocity_y *= -1;
        }

        self.box_x += self.velocity_x;
        self.box_y += self.velocity_y;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&mut self, frame: &mut [u8]) {
        profiling::scope!("Main Thread");
        let mut buffer = ObjectBuffer::new();
        self.theta += 0.01;

        let _move_origin = Matrix::move_matrix(-0., -0., 0.);
        let _mat = Vector3::new(1., 1., 1.).to_rotation_matrix(self.theta);
        let _move_back = Matrix::move_matrix(0., 0., -7.5);
        let _mat = ((&_move_back * &_mat).unwrap() * _move_origin).unwrap();


        let mut new_obj = self.obj.clone();
        new_obj.mul_matrix(&_mat);

        buffer.add_object(new_obj);
        let _buf = self.camera.render(WIDTH, HEIGHT, &buffer);

        frame.copy_from_slice(&_buf.display);
        profiling::finish_frame!();
    }
}
