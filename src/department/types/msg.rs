#[derive(Clone, PartialEq)]
pub enum DognutOption {
    StartEncode = 1,
    EncoderStarted = 2,
}


#[derive(Clone)]
pub enum TransferMsg {
    RenderedData(Vec<u8>),
    DogOpt(DognutOption),
    QuitThread,
    Test(u32)
}
