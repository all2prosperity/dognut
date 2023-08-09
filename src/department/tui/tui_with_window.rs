use std::error::Error;
use std::io::{Stdout, stdout, Write};
use std::time::Duration;
use crossterm::{event, execute};
use crossterm::event::Event;
use crossterm::terminal::{ClearType,disable_raw_mode, enable_raw_mode, EnterAlternateScreen};
use game_loop::TimeTrait;
use crate::department::common::self_type;
use crate::department::control::camera_controller::CameraController;
use crate::department::model::triangle_resources::TriangleResources;
use crate::department::pipeline::rasterizer::RasterRunner;
use crate::department::preview::homo_transformation::HomoTransform;
use crate::department::preview::output_buffer::OutputBuffer;
use crate::department::preview::vector::Vector3;
use crate::department::tui::game_loop;
use crate::department::types::msg::TransferMsg;
use crate::department::types::multi_sender::MultiSender;

pub struct TuiWinApp {
    pub raster: RasterRunner,
    stdout: Stdout,
    theta: f32,
    camera_controller: CameraController,
    gpu: Option<self_type::StateImp>,
    fps: u32,
    time_step: Duration,
    res: TriangleResources,
    ms: MultiSender<TransferMsg>,
}


impl TuiWinApp {
    pub fn new(raster: RasterRunner, res: TriangleResources, ms: MultiSender<TransferMsg>) -> Self {
        Self {
            raster,
            stdout: stdout(),
            theta: 0.,
            gpu: None,
            camera_controller: CameraController::new(2.0, 0.2, true),
            fps: 30,
            time_step: Duration::from_nanos(1_000_000_000 / 30 as u64),
            res,
            ms,
        }
    }

    pub fn run(mut self, state: Option<self_type::StateImp>) -> Result<(), Box<dyn Error>> {
        enable_raw_mode()?;

        execute!(self.stdout, crossterm::cursor::Hide);
        execute!(self.stdout, EnterAlternateScreen, event::EnableMouseCapture);
        execute!(self.stdout, crossterm::terminal::Clear(ClearType::All));

        let dimension = (256, 79);
        self.gpu = state;
        let fps = self.fps.clone();
        let _lop = game_loop(self, fps, 0.1, |g| {
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
            g.game.draw((dimension.0 as u32, dimension.1 as u32));

            let st = g.game.time_step.as_secs_f64() - game_loop::Time::now().sub(&g.current_instant());
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
            let mut out_buf = OutputBuffer::new(dim.0 as u32, dim.1 as u32, true);
            out_buf.stdout = Some(&mut self.stdout);
            let out = gpu.render(true);
            out_buf.display.copy_from_slice(&out.1.unwrap());
            self.ms.win.try_send(TransferMsg::RenderedData(out.0)).unwrap();
            out_buf.queue_to_stdout();
            drop(out_buf);
            self.stdout.flush().unwrap();
            return;
        }


        let _now = std::time::Instant::now();
        let mut out_buf = OutputBuffer::new(dim.0 as u32, dim.1 as u32, true);
        out_buf.stdout = Some(&mut self.stdout);
        self.raster.set_model(HomoTransform::rotation_matrix(&Vector3::from_xyz(0., 1., 0.), self.theta) * HomoTransform::scale((1.5, 1.5, 1.5)));
        self.raster.render_frame(&self.res, &mut out_buf);
        out_buf.queue_to_stdout();
        let data = out_buf.display.clone();
        drop(out_buf);
        self.stdout.flush().unwrap();

        self.raster.encoder_tx.enc.send(TransferMsg::RenderedData(data)).unwrap();
    }
}

impl Drop for TuiWinApp {
    fn drop(&mut self) {
        execute!(self.stdout, crossterm::terminal::Clear(ClearType::All));
        execute!(self.stdout, crossterm::terminal::LeaveAlternateScreen, event::DisableMouseCapture);
        execute!(self.stdout, crossterm::cursor::Show);
        disable_raw_mode().unwrap();
    }
}
