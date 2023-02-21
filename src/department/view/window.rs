#![deny(clippy::all)]
#![forbid(unsafe_code)]

use std::time::Duration;

use game_loop::{game_loop, Time, TimeTrait};
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use pixels::wgpu::Color;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use crate::department::common::constant::{HEIGHT, WIDTH};
use crate::department::common::self_type;
use crate::department::Game;
use crate::wgpu::wgpu_helper::State;

pub const FPS: usize = 60;
pub const TIME_STEP: Duration = Duration::from_nanos(1_000_000_000 / FPS as u64);


/// Representation of the application state. In this example, a box will bounce around the screen.


pub async fn run(rgba_tx: crossbeam_channel::Sender<Vec<u8>>) -> Result<(), Error> {
    let camera = self_type::camera_instance();
    let state = State::new(PhysicalSize { width: WIDTH, height: HEIGHT }, camera).await;

    let event_loop = EventLoop::new();
    let _input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Main Window")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    window.set_outer_position(winit::dpi::Position::from(winit::dpi::PhysicalPosition{x:100, y: 100}));

    let id = window.id();

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    pixels.set_clear_color(Color::WHITE);

    let _frames: std::collections::VecDeque<Vec<u8>> = std::collections::VecDeque::new();

    let game = Game::new(pixels, state, id, false);

    let mut index = 0;

    game_loop(event_loop, window, game, FPS as u32, 0.1,
              |g| {
                  if !g.game.paused {
                      g.game.state.update(std::time::Duration::from_secs_f64(g.last_frame_time()));
                  }
              },
              move |g| {
                  let out = g.game.state.render();
                  g.game.pixels.get_frame_mut().copy_from_slice(&out);
                  if let Err(e) = rgba_tx.try_send(out) {
                      error!("send raw rgba fail: reason {:?}", e);
                  }

                  log::info!("send rgba frame to encoder index {}", index);
                  index += 1;

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
                      Event::WindowEvent { ref event, .. } => {
                          match event {
                              WindowEvent::CloseRequested | WindowEvent::KeyboardInput {
                                  input:
                                  KeyboardInput {
                                      state: ElementState::Pressed,
                                      virtual_keycode: Some(VirtualKeyCode::Escape),
                                      ..
                                  }, ..
                              } => {
                                  g.exit();
                                  return;
                              }
                              WindowEvent::Resized(physical_size) => {
                                  g.game.state.resize(*physical_size);
                                  g.game.pixels.resize_surface(physical_size.width, physical_size.height);
                              }
                              WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                  g.game.state.resize(**new_inner_size);
                                  g.game.pixels.resize_surface(new_inner_size.width, new_inner_size.height);
                              }
                              WindowEvent::KeyboardInput { input, .. } => {
                                  let ret = g.game.state.camera_controller.process_keyboard(input.virtual_keycode.unwrap(), input.state);
                                  if !ret {
                                      g.exit();
                                      return;
                                  }
                              }
                              WindowEvent::ModifiersChanged(ms) => {
                                  g.game.state.camera_controller.ctrl_pressed = ms.ctrl();
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

    Ok(())
}
