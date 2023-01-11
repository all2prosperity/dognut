use std::error::Error;
use std::io::{Stdout, stdout, Write};
use std::time::Duration;
use crossterm;
use crossterm::{event, execute, terminal};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{ClearType, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, SetSize, size};


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
    gpu: Option<State>,
}


impl TuiApp {
    pub fn new(raster: RasterRunner) -> Self {
        Self { raster, stdout: stdout(), theta: 0., gpu: None }
    }

    pub fn run(mut self, res: TriangleResources, state: State) -> Result<(), Box<dyn Error>> {
        enable_raw_mode()?;
        execute!(self.stdout, crossterm::cursor::Hide);
        execute!(self.stdout, EnterAlternateScreen, event::EnableMouseCapture);
        execute!(self.stdout, crossterm::terminal::Clear(ClearType::All));

        let dimension = (256,79);
        self.gpu = Some(state);

        loop {
            self.theta += 0.1;
            self.draw((dimension.0 as u32, dimension.1 as u32), &res);
            if let Ok(ready) = event::poll(Duration::from_millis(1000 / 25)) {
                if ready {
                    match event::read()? {
                        Event::FocusGained => {}
                        Event::FocusLost => {}
                        Event::Key(k) => {
                            match k.code {
                                KeyCode::Backspace => {}
                                KeyCode::Enter => {}
                                KeyCode::Left => {}
                                KeyCode::Right => {}
                                KeyCode::Up => {}
                                KeyCode::Down => {}
                                KeyCode::Home => {}
                                KeyCode::End => {}
                                KeyCode::PageUp => {}
                                KeyCode::PageDown => {}
                                KeyCode::Tab => {}
                                KeyCode::BackTab => {}
                                KeyCode::Delete => {}
                                KeyCode::Insert => {}
                                KeyCode::F(_) => {}
                                KeyCode::Char(c) => {
                                    if c == 'q' {
                                        break;
                                    }
                                }
                                KeyCode::Null => {}
                                KeyCode::Esc => {
                                    break;
                                }
                                KeyCode::CapsLock => {}
                                KeyCode::ScrollLock => {}
                                KeyCode::NumLock => {}
                                KeyCode::PrintScreen => {}
                                KeyCode::Pause => {}
                                KeyCode::Menu => {}
                                KeyCode::KeypadBegin => {}
                                KeyCode::Media(_) => {}
                                KeyCode::Modifier(_) => {}
                                _ => {}
                            }
                        }
                        Event::Mouse(_) => {}
                        Event::Paste(_) => {}
                        Event::Resize(w, h) => {
                            self.draw((w as u32, h as u32), &res);
                        }
                    }
                }
            }
            execute!(self.stdout, terminal::Clear(ClearType::All));
        }


        Ok(())
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