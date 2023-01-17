use std::sync::mpsc::Receiver;
use std::time::Duration;
use tokio;
use tokio::runtime::Runtime;
use tui::backend::Backend;
use tui::Frame;

pub struct TermRenderer {
    keyboard_rx: Receiver<String>,
    rt: Runtime,
}

impl TermRenderer {
    pub fn new_with(rx: Receiver<String>) -> Self {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        TermRenderer {
            keyboard_rx: rx,
            rt: runtime,
        }
    }

    pub fn run(self) {
        self.rt.block_on(async {
            let _ = &self.update();
        });
    }

    async fn update(&self) {
        loop {
            if let Ok(_s) = self.keyboard_rx.recv_timeout(Duration::from_secs(1)) {}
        }
    }
}

fn ui<B: Backend>(_f: &mut Frame<B>, _render: &TermRenderer) {}
