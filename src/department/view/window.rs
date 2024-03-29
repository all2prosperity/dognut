#![deny(clippy::all)]
#![forbid(unsafe_code)]

use std::time::Duration;

use game_loop::{game_loop, Time, TimeTrait};
use log::{error, info};
use pixels::{Error, Pixels, SurfaceTexture};
use pixels::wgpu::Color;
use winit::dpi::{LogicalSize};
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use crate::department::types::msg::{TransferMsg, DognutOption};
use crate::department::types::multi_sender::MultiSender;
use crate::department::common::constant::{HEIGHT, WHOLE_WIDTH, WIDTH};
use crate::department::common::self_type;
use crate::department::Game;
use crate::wgpu::wgpu_helper::State;

pub const FPS: usize = 30;
pub const TIME_STEP: Duration = Duration::from_nanos(1_000_000_000 / FPS as u64);


/// Representation of the application state. In this example, a box will bounce around the screen.
///

pub async fn run(win_receiver: crossbeam_channel::Receiver<TransferMsg>, ms: MultiSender<TransferMsg>, split: bool) -> Result<(), Error> {
    let setting_width = if split { WHOLE_WIDTH } else { WIDTH };
    let camera = self_type::camera_instance(setting_width, HEIGHT);
    let state = State::new(LogicalSize { width: setting_width, height: HEIGHT }, camera).await;

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
    let scale_factor = window.scale_factor();

    let mut pixels = {
        let window_size = window.inner_size();
        info!("scale factor is {}", scale_factor);
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    pixels.set_clear_color(Color::WHITE);
    let game = Game::new(pixels, state, id, false);

    let mut start_enc_render = false;

    let mut index = 0;

    game_loop(event_loop, window, game, FPS as u32, 0.1,
        |g| {
            if !g.game.paused {
                g.game.state.update(std::time::Duration::from_secs_f64(g.last_frame_time()));
            }
        },
        move |g| {
            if let Ok(msg) = win_receiver.try_recv() {
                if let TransferMsg::DogOpt(_code) = msg {
                    if _code == DognutOption::EncoderStarted {
                        start_enc_render = true;
                    }
                }
            }


            let out = g.game.state.render(false);
            if split {
               let (this, that) = crate::util::split_screen(&out.0, (WHOLE_WIDTH, HEIGHT), (WIDTH, HEIGHT));
                g.game.pixels.get_frame_mut().copy_from_slice(&that.as_slice());
                if start_enc_render {
                    if let Err(e) = ms.enc.try_send(TransferMsg::RenderedData(this)) {
                        error!("send raw rgba fail: reason {:?}", e);
                    }
                    //info!("send rgba frame to encoder index {}", index);
                    index += 1;
                }
            }else {
                g.game.pixels.get_frame_mut().copy_from_slice(&out.0.as_slice());
                if start_enc_render {
                    if let Err(e) = ms.enc.try_send(TransferMsg::RenderedData(out.0)) {
                        error!("send raw rgba fail: reason {:?}", e);
                    }
                    //info!("send rgba frame to encoder index {}", index);
                    index += 1;
                }
            }






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
                            g.game.state.resize(*physical_size, scale_factor);
                            g.game.pixels.resize_surface(physical_size.width, physical_size.height);
                        }
                        WindowEvent::ScaleFactorChanged {scale_factor, new_inner_size } => {
                            g.game.state.resize(**new_inner_size, *scale_factor);
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
                Event::DeviceEvent {  .. } => {
                    //g.game.state.input(event);
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
