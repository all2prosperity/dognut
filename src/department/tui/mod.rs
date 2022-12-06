use std::error::Error;
use std::io::{Stdout, stdout, Write};
use std::time::Duration;
use crossterm;
use crossterm::{event, execute, queue, terminal};
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::terminal::{ClearType, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, size};
use tui::backend::CrosstermBackend;
use tui::Terminal;
use crate::department::model::object_loader::ObjectLoader;
use crate::department::model::triangle_resources::TriangleResources;
use crate::department::pipeline::rasterizer::RasterRunner;
use crate::department::preview::homo_transformation::HomoTransform;
use crate::department::preview::output_buffer::OutputBuffer;
use crate::department::preview::vector::Vector3;

pub mod term;



pub struct TuiApp {
    pub raster: RasterRunner,
    stdout: Stdout,
    theta: f32,
}



impl TuiApp {
    pub fn new(raster: RasterRunner) -> Self {
        Self { raster, stdout: stdout(), theta: 0. }
    }

    pub fn run(mut self, res: TriangleResources) -> Result<(), Box<dyn Error>>{
        enable_raw_mode()?;
        execute!(self.stdout, crossterm::cursor::Hide);
        execute!(self.stdout, EnterAlternateScreen, event::EnableMouseCapture);
        execute!(self.stdout, crossterm::terminal::Clear(ClearType::All));
        let mut dimension = size()?;
        loop {
            self.theta += 0.1;
            self.draw((dimension.0 as u32, dimension.1 as u32), &res);
            if let Ok(ready) = event::poll(Duration::from_millis(1000/25)) {
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
        let mut out_buf = OutputBuffer::new(dim.0 as u32, dim.1 as u32, true);
        out_buf.stdout = Some(&mut self.stdout);
        self.raster.set_model(HomoTransform::rotation_matrix(&Vector3::from_xyz(0.,1.,0.), self.theta));
        self.raster.render_frame(res, &mut out_buf);
        out_buf.queue_to_stdout();
        drop(out_buf);
        self.stdout.flush().unwrap();
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