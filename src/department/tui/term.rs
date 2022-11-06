use std::io;
use tui::{backend::CrosstermBackend, Terminal};

pub struct TermRenderer<W> {
    terminal: Terminal<CrosstermBackend<W>>
}


impl TermRenderer<W> {
    pub fn new() ->Self {
        let stdout = std::io::stdout();
        let backend=  CrosstermBackend::new(stdout);
        TermRenderer{
            terminal: Terminal::new(backend).unwrap()
        }
    }
}