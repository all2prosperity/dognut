#[derive(Clone, PartialEq)]
pub enum DognutOption {
    StartEncode = 1,
}


#[derive(Clone)]
pub enum TransferMsg {
    RenderPc(Vec<u8>),
    DogOpt(DognutOption),
    Test(u32)
}
