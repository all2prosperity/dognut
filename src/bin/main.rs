use dognut;

use tui::backend::CrosstermBackend;
use tui::Terminal;
use department::preview::*;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use dognut::department::preview::matrix;
use dognut::department::preview::vector::Vector3;



fn main() {
    let stdout = std::io::stdout();
    let backend=  CrosstermBackend::new(stdout);
    let tem = Terminal::new(backend).unwrap();

    pollster::block_on(state::run());
}