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
use crate::department::preview::homo_transformation::{HomoTransform, Transform};
use crate::department::view::camera::Camera;
use crate::department::model::object_buffer::ObjectBuffer;
use crate::department::preview::matrix::{HMat, Matrix};
use crate::department::preview::vector::Vector3;
use crate::department::preview::position::Pos3;
use crate::department::model::render_object::RenderObject;
use crate::department::model::object_loader::ObjectLoader;
use crate::department::model::triangle_resources::TriangleResources;
use crate::department::types::msg::TransferMsg;

use crossbeam_channel::Receiver;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;

/// Representation of the application state. In this example, a box will bounce around the screen.


pub fn run(render_recv: Receiver<TransferMsg>) -> Result<(), Error> {
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

    let mut frames: std::collections::VecDeque<Vec<u8>> = std::collections::VecDeque::new();

    event_loop.run(move |event, _, control_flow| {
        if let Ok(ret) = render_recv.try_recv() {
            println!("recv ok!");
            match ret {
                TransferMsg::RenderPc(frame) => {
                    frames.push_back(frame);
                },
                _ => (),
            }
        }

        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            if frames.len() > 1 {
                pixels.get_frame_mut().copy_from_slice(&frames.pop_front().unwrap());
            }
            else if frames.len() == 1 {
                pixels.get_frame_mut().copy_from_slice(&frames[0]);
            }
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

            // if input.key_pressed(VirtualKeyCode::A) {
            //     world.camera.move_view(VirtualKeyCode::A);
            // }
            // else if input.key_pressed(VirtualKeyCode::D) {
            //     world.camera.move_view(VirtualKeyCode::D);
            // }
            // else if input.key_pressed(VirtualKeyCode::W) {
            //     world.camera.move_view(VirtualKeyCode::W);
            // }
            // else if input.key_pressed(VirtualKeyCode::S) {
            //     world.camera.move_view(VirtualKeyCode::S);
            // }
            // else if input.key_pressed(VirtualKeyCode::Q) {
            //     world.camera.move_view(VirtualKeyCode::Q);
            // }
            // else if input.key_pressed(VirtualKeyCode::E) {
            //     world.camera.move_view(VirtualKeyCode::E);
            // }
            //
            // // Resize the window
            // if let Some(size) = input.window_resized() {
            //     pixels.resize_surface(size.width, size.height);
            // }
            //
            // // Update internal state and request a redraw
            // world.update();
            window.request_redraw();
        }
    });
}

