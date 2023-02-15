use crossbeam_channel::Sender;

#[derive(Clone)]
pub struct MultiSender<T: Clone> {
    pub net: Sender<T>,
    pub wgpu: Sender<T>,
    pub win: Sender<T>,
}

impl<T: Clone> MultiSender<T> {
    pub fn new(net: Sender<T>, wgpu: Sender<T>, win: Sender<T>) -> Self {
        Self {
            net, wgpu, win
        }
    }
}
