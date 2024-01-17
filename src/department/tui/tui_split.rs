use std::io::{Stdout, stdout, Write};
use std::time::Duration;
use crossterm::{event, execute};
use crossterm::event::Event;
use crossterm::terminal::{ClearType};
use game_loop::TimeTrait;
use crate::department::preview::output_buffer::OutputBuffer;
use crate::department::types::msg::TransferMsg;
use crate::department::types::multi_sender::MultiSender;
use super::game_loop;


const TUI_WIDE_WIDTH: u32 = 256 * 2;
const TUI_SPLIT_WIDTH: u32 = 256;
const TUI_SPLIT_HEIGHT: u32 = 79;

pub struct TuiSplitApp {
    stdout: Stdout,
    theta: f32,
    camera_controller: crate::department::control::camera_controller::CameraController,
    gpu: Option<crate::department::common::self_type::StateImp>,
    ms: MultiSender<TransferMsg>,
}


static FPS: u32 = 30;
static TIME_STEP: Duration = Duration::from_nanos(1_000_000_000 / FPS as u64);


impl TuiSplitApp {
    pub fn new(ms: MultiSender<TransferMsg>) -> Self {
        Self {
            stdout: stdout(),
            theta: 0.,
            gpu: None,
            camera_controller: crate::department::control::camera_controller::CameraController::new(2.0, 0.2, true),
            ms,
        }
    }

    pub async fn run(mut self) -> Result<(), Box<dyn std::error::Error>> {
        crossterm::terminal::enable_raw_mode()?;

        execute!(self.stdout, crossterm::cursor::Hide);
        execute!(self.stdout, crossterm::terminal::EnterAlternateScreen, event::EnableMouseCapture);
        execute!(self.stdout, crossterm::terminal::Clear(ClearType::All));

        let cam = crate::department::common::self_type::camera_instance(TUI_SPLIT_WIDTH, TUI_SPLIT_HEIGHT);
        let gpu = crate::wgpu::wgpu_helper::State::new(winit::dpi::LogicalSize { width: TUI_WIDE_WIDTH, height: TUI_SPLIT_HEIGHT }, cam).await;
        self.gpu = Some(gpu);


        let _lop = game_loop(self, FPS, 0.1, |g| {
            // update
            g.game.update(g.last_frame_time());
        }, |g| {
            let mut should_exit = false;
            loop {
                if let Ok(ready) = event::poll(Duration::from_secs(0)) {
                    if ready {
                        let event_res = event::read();
                        if event_res.is_ok() {
                            match event_res.unwrap() {
                                Event::FocusGained => {}
                                Event::FocusLost => {}
                                Event::Key(k) => {
                                    if !g.game.camera_controller.process_tui_keyboard(&k) {
                                        should_exit = true;
                                    }
                                }
                                Event::Mouse(_) => {}
                                Event::Paste(_) => {}
                                Event::Resize(w, h) => {
                                    println!("terminal window update to new size {} {}", w, h);
                                    //self.draw((w as u32, h as u32), &res);
                                }
                            }
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            if should_exit {
                g.game.ms.enc.try_send(TransferMsg::QuitThread).unwrap();
                g.game.ms.win.try_send(TransferMsg::QuitThread).unwrap();
                g.exit();
            }
            // execute!(g.game.stdout, terminal::Clear(ClearType::All));
            g.game.draw((TUI_SPLIT_WIDTH, TUI_SPLIT_HEIGHT));

            let st = TIME_STEP.as_secs_f64() - game_loop::Time::now().sub(&g.current_instant());
            if st > 0. {
                std::thread::sleep(Duration::from_secs_f64(st));
            }
        });


        Ok(())
    }

    pub fn update(&mut self, last_frame_time: f64) {
        if let Some(ref mut gpu) = self.gpu {
            gpu.update_outside(&mut self.camera_controller, Duration::from_secs_f64(last_frame_time));
        }
    }

    pub fn draw(&mut self, dim: (u32, u32)) {
        if let Some(ref mut gpu) = self.gpu {
            let mut out_buf = OutputBuffer::new(dim.0 as u32, dim.1 as u32, false);
            out_buf.stdout = Some(&mut self.stdout);
            let out = gpu.render(false);
            let (this, that) = crate::util::split_screen(&out.0, (TUI_WIDE_WIDTH, TUI_SPLIT_HEIGHT), (TUI_SPLIT_WIDTH, TUI_SPLIT_HEIGHT));
            out_buf.display.copy_from_slice(&that);
            self.ms.enc.try_send(TransferMsg::RenderedData(this)).unwrap();
            out_buf.queue_to_stdout();
            drop(out);
            drop(out_buf);
            self.stdout.flush().unwrap();
            return;
        }
    }
}

impl Drop for TuiSplitApp {
    fn drop(&mut self) {
        execute!(self.stdout, crossterm::terminal::Clear(ClearType::All));
        execute!(self.stdout, crossterm::terminal::LeaveAlternateScreen, event::DisableMouseCapture);
        execute!(self.stdout, crossterm::cursor::Show);
        crossterm::terminal::disable_raw_mode().unwrap();
    }
}
