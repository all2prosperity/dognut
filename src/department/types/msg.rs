use winit::event::WindowEvent;


#[derive(Clone)]
pub enum TransferMsg {
    RenderPc(Vec<u8>),
    Test(u32)
}
