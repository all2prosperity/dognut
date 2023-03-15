use std::thread::JoinHandle;
use std::time::Instant;
use crate::department::types::msg::{DognutOption, TransferMsg};
use crate::department::types::multi_sender::MultiSender;
use image::ImageEncoder;
use log::info;

#[cfg(feature = "rtc")]
pub mod encode;
#[cfg(feature = "rtc")]
pub mod decode;


pub struct ImgEncoder {
    rx: crossbeam_channel::Receiver<TransferMsg>,
    ms: MultiSender<TransferMsg>,
    dimension: (u32, u32),
}


impl ImgEncoder {
    pub fn new(rx: crossbeam_channel::Receiver<TransferMsg>, ms: MultiSender<TransferMsg>, dimension: (u32, u32)) -> Self {
        Self { rx, ms, dimension }
    }

    pub fn run(rx: crossbeam_channel::Receiver<TransferMsg>, ms: MultiSender<TransferMsg>, dimension: (u32, u32)) -> JoinHandle<()> {
        let handle = std::thread::Builder::new().name("dognut_image_encoding".into()).spawn(move || {
            let encoder = Self::new(rx, ms, dimension);
            encoder.run_encoding_pipeline();
            return ();
        }).unwrap();

        return handle;
    }

    pub fn run_encoding_pipeline(mut self) {
        // loop {
        //     let msg = self.rx.recv().unwrap();
        //     match msg {
        //         TransferMsg::RenderedData(_) => {}
        //         TransferMsg::DogOpt(code) => {
        //             if code == DognutOption::StartEncode {
        //                 self.ms.win.send(TransferMsg::DogOpt(DognutOption::EncoderStarted)).expect("must send ok");
        //                 break;
        //             }
        //         }
        //         TransferMsg::QuitThread => {
        //             info!("encoder thread quit on msg");
        //             return;
        //         }
        //         _ => {}
        //     }
        // }

        self.ms.win.send(TransferMsg::DogOpt(DognutOption::EncoderStarted)).expect("must send ok");

        let mut index = 0;
        loop {
            if let Ok(msg) = self.rx.recv() {
                match msg {
                    TransferMsg::RenderedData(data) => {
                        let instant = Instant::now();
                        let mut out = Vec::new();
                        image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, 50)
                            .encode(data.as_slice(), self.dimension.0, self.dimension.1, image::ColorType::Rgba8)
                            .expect("encode image failed");
                        info!("encode image cost {:?} with size {}", instant.elapsed(), out.len());
                    }
                    TransferMsg::QuitThread => {
                        info!("img encoder thread quit on msg");
                        return;
                    }
                    _ => {}
                }
            }
        }
    }
}
