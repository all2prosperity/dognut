use std::io;
use tui::{backend::CrosstermBackend, Frame, Terminal, TerminalOptions};
use tui::backend::Backend;

pub struct TermShader<W> {
    terminal: Terminal<CrosstermBackend<W>>
}


impl TermShader<W> {
    pub fn new() ->Self {
        let stdout = std::io::stdout();
        let backend=  CrosstermBackend::new(stdout);
        TermShader{
            terminal: Terminal::new(backend).unwrap()
        }
    }


    pub fn update<B:Backend>(mut self) {
        self.terminal.draw(|f| ui(f, &self))?;

    }
}

fn ui<B:Backend>(f: &mut Frame<B>, render: &TermShader<B>) {

}