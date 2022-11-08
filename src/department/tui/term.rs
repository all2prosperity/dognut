
use std::io;
use std::io::Stdout;
use std::sync::mpsc::Receiver;
use std::time::Duration;
use tui::{backend::CrosstermBackend, Frame, Terminal, TerminalOptions};
use tui::backend::Backend;
use tokio;
use tokio::runtime::Runtime;

pub struct TermRenderer
{
    keyboard_rx: Receiver<String>,
    rt: Runtime
}


impl TermRenderer where {
    pub fn new_with(rx: Receiver<String>) ->Self {
        let runtime = tokio::runtime::Builder::new_current_thread().build().unwrap();
        TermRenderer {
            keyboard_rx: rx,
            rt: runtime
        }
    }

    pub fn run(mut self) {
        self.rt.block_on(async {
            &self.update();
        });
    }

    async fn update(&self) {
        loop {
            if let Ok(s) = self.keyboard_rx.recv_timeout(Duration::from_secs(1)) {
                
            }
        }
    }
}

fn ui<B:Backend>(f: &mut Frame<B>, render: &TermRenderer) {

}