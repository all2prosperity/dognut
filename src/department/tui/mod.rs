use std::error::Error;
use std::io::{Stdout, stdout, Write};
use std::time::Duration;
use crate::department::common::self_type;
use crossterm;
use crossterm::{event, execute, terminal};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{ClearType, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, SetSize, size};
use game_loop::{GameLoop, Time, TimeTrait};
use crate::department::control::camera_controller::CameraController;



use crate::department::model::triangle_resources::TriangleResources;
use crate::department::pipeline::rasterizer::RasterRunner;
use crate::department::preview::homo_transformation::HomoTransform;
use crate::department::preview::output_buffer::OutputBuffer;
use crate::department::preview::vector::Vector3;
use crate::wgpu::wgpu_helper::State;

pub mod term;


pub struct TuiApp {
    pub raster: RasterRunner,
    stdout: Stdout,
    theta: f32,
    camera_controller: CameraController,
    gpu: Option<self_type::StateImp>,
}

static FPS:u32 = 30;

static TIME_STEP: Duration = Duration::from_nanos(1_000_000_000 / FPS as u64);

fn game_loop<G, U, R>(game: G, updates_per_second: u32, max_frame_time: f64, mut update: U, mut render: R) -> GameLoop<G, game_loop::Time, ()>
    where U: FnMut(&mut game_loop::GameLoop<G, game_loop::Time, ()>),
          R: FnMut(&mut game_loop::GameLoop<G, game_loop::Time, ()>),
{
    let mut game_loop = game_loop::GameLoop::new(game, updates_per_second, max_frame_time, ());

    while game_loop.next_frame(&mut update, &mut render) {}

    game_loop
}

impl TuiApp {
    pub fn new(raster: RasterRunner) -> Self {
        Self { raster, stdout: stdout(), theta: 0., gpu: None, camera_controller: CameraController::new(2.0, 0.2, true)}
    }

    pub fn run(mut self, res: TriangleResources, state: Option<self_type::StateImp>) -> Result<(), Box<dyn Error>> {
        enable_raw_mode()?;

        execute!(self.stdout, crossterm::cursor::Hide);
        execute!(self.stdout, EnterAlternateScreen, event::EnableMouseCapture);
        execute!(self.stdout, crossterm::terminal::Clear(ClearType::All));

        let dimension = (256,79);
        self.gpu = state;

        let lop = game_loop(self, FPS, 0.1, |g| {
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
                        }else {
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
                g.exit();
            }
           // execute!(g.game.stdout, terminal::Clear(ClearType::All));
            g.game.draw((dimension.0 as u32, dimension.1 as u32), &res);

            let st = TIME_STEP.as_secs_f64() - Time::now().sub(&g.current_instant());
            if st > 0. {
                std::thread::sleep(Duration::from_secs_f64(st));
            }
        });
        Ok(())
    }

    pub fn update(&mut self, last_frame_time: f64)  {
        if let Some(ref mut gpu) = self.gpu {
            gpu.update_outside(&mut self.camera_controller,Duration::from_secs_f64(last_frame_time));
        }
    }


    pub fn draw(&mut self, dim: (u32, u32), res: &TriangleResources) {
        if let Some(ref mut gpu) = self.gpu {
            let mut out_buf = OutputBuffer::new(dim.0 as u32, dim.1 as u32, true);
            out_buf.stdout = Some(&mut self.stdout);
            let out = gpu.render();
            out_buf.display.copy_from_slice(&out);
            out_buf.queue_to_stdout();
            drop(out_buf);
            self.stdout.flush().unwrap();
            return ;
        }


        let now = std::time::Instant::now();
        let mut out_buf = OutputBuffer::new(dim.0 as u32, dim.1 as u32, true);
        out_buf.stdout = Some(&mut self.stdout);
        self.raster.set_model(HomoTransform::rotation_matrix(&Vector3::from_xyz(0., 1., 0.), self.theta) * HomoTransform::scale((1.5, 1.5, 1.5)));
        self.raster.render_frame(res, &mut out_buf);
        out_buf.queue_to_stdout();
        let data = out_buf.display.clone();
        drop(out_buf);
        self.stdout.flush().unwrap();

        println!("this frame cost {} milli sec", now.elapsed().as_millis());
        self.raster.encoder_tx.send(data).unwrap();
    }
}

impl Drop for TuiApp {
    fn drop(&mut self) {
        execute!(self.stdout, terminal::Clear(ClearType::All));
        execute!(self.stdout, terminal::LeaveAlternateScreen, event::DisableMouseCapture);
        execute!(self.stdout, crossterm::cursor::Show);
        disable_raw_mode().unwrap();
    }
}
