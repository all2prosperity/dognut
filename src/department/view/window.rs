#![deny(clippy::all)]
#![forbid(unsafe_code)]

use std::time::Duration;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use pixels::wgpu::Color;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use crate::department::common::constant::{WIDTH, HEIGHT};


use crate::department::types::msg::TransferMsg;

use crossbeam_channel::Receiver;
use game_loop::{game_loop, Time, TimeTrait};
use crate::department::Game;
use crate::wgpu::wgpu_helper::State;
use crate::department::types::multi_sender::MultiSender;

pub const FPS: usize = 60;
pub const TIME_STEP: Duration = Duration::from_nanos(1_000_000_000 / FPS as u64);


/// Representation of the application state. In this example, a box will bounce around the screen.


pub async fn run(render_recv: Receiver<TransferMsg>, ms: MultiSender<TransferMsg>) -> Result<(), Error> {
    let mut state = State::new(PhysicalSize { width: WIDTH, height: HEIGHT }).await;

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

    let id = window.id();

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    pixels.set_clear_color(Color::WHITE);

    let mut frames: std::collections::VecDeque<Vec<u8>> = std::collections::VecDeque::new();

    let game = Game::new(pixels, state, id, false);
       
    game_loop(event_loop, window, game, FPS as u32, 0.1,
               |g| {
                  if !g.game.paused {
                      g.game.state.update(std::time::Duration::from_secs_f64(g.last_frame_time()));
                  }
              },
               |g| {
                  let out = g.game.state.render();
                  g.game.pixels.get_frame_mut().copy_from_slice(&out);

                  if let Err(err) = g.game.pixels.render() {
                      error!("pixels.render() failed: {err}");
                      g.exit();
                  }

                  let st = TIME_STEP.as_secs_f64() - Time::now().sub(&g.current_instant());
                  if st > 0. {
                      std::thread::sleep(Duration::from_secs_f64(st));
                  }
              },
              move |g, event| {
                  match event {
                      Event::NewEvents(_) => {}
                      Event::WindowEvent { ref event, ..} => {
                          match event {
                              WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                                  input:
                                  KeyboardInput {
                                      state: ElementState::Pressed,
                                      virtual_keycode: Some(VirtualKeyCode::Escape),
                                      ..
                                  }, ..
                              } => { g.exit(); return ;},
                              WindowEvent::Resized(physical_size) => {
                                  g.game.state.resize(*physical_size);
                                  g.game.pixels.resize_surface(physical_size.width, physical_size.height);
                              }
                              WindowEvent::ScaleFactorChanged {new_inner_size, ..} => {
                                  g.game.state.resize(**new_inner_size);
                                  g.game.pixels.resize_surface(new_inner_size.width, new_inner_size.height);
                              }
                              WindowEvent::KeyboardInput { input, .. } => {
                                  g.game.state.camera_controller.process_keyboard(input.virtual_keycode.unwrap(), input.state);
                              }
                              _ => {}
                          }
                      }
                      Event::DeviceEvent { ref event, .. } => {
                          g.game.state.input(event);
                      }
                      Event::RedrawRequested(_) => {
                          //g.game.pixels.window_pos_to_pixel()
                      }
                      _ => {}
                  }

              },
    );

    // event_loop.run(move |event, _, control_flow| {
    //     if let Ok(ret) = render_recv.try_recv() {
    //         match ret {
    //             TransferMsg::RenderPc(frame) => {
    //                 frames.push_back(frame);
    //             }
    //             _ => (),
    //         }
    //     }
    //
    //     // Draw the current frame
    //     if let Event::RedrawRequested(_) = event {
    //         if frames.len() > 1 {
    //             pixels.get_frame_mut().copy_from_slice(&frames.pop_front().unwrap());
    //         } else if frames.len() == 1 {
    //             pixels.get_frame_mut().copy_from_slice(&frames[0]);
    //         }
    //         if pixels
    //             .render()
    //             .map_err(|e| error!("pixels.render() failed: {}", e))
    //             .is_err()
    //         {
    //             *control_flow = ControlFlow::Exit;
    //             return;
    //         }
    //     }
    //
    //     // Handle input events
    //     if input.update(&event) {
    //         // Close events
    //         if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
    //             *control_flow = ControlFlow::Exit;
    //             return;
    //         }
    //
    //         // if input.key_pressed(VirtualKeyCode::A) {
    //         //     world.camera.move_view(VirtualKeyCode::A);
    //         // }
    //         // else if input.key_pressed(VirtualKeyCode::D) {
    //         //     world.camera.move_view(VirtualKeyCode::D);
    //         // }
    //         // else if input.key_pressed(VirtualKeyCode::W) {
    //         //     world.camera.move_view(VirtualKeyCode::W);
    //         // }
    //         // else if input.key_pressed(VirtualKeyCode::S) {
    //         //     world.camera.move_view(VirtualKeyCode::S);
    //         // }
    //         // else if input.key_pressed(VirtualKeyCode::Q) {
    //         //     world.camera.move_view(VirtualKeyCode::Q);
    //         // }
    //         // else if input.key_pressed(VirtualKeyCode::E) {
    //         //     world.camera.move_view(VirtualKeyCode::E);
    //         // }
    //         //
    //         // // Resize the window
    //         // if let Some(size) = input.window_resized() {
    //         //     pixels.resize_surface(size.width, size.height);
    //         // }
    //         //
    //         // // Update internal state and request a redraw
    //         // world.update();
    //         window.request_redraw();
    //     }
    // });
}

