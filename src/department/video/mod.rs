use std::thread::JoinHandle;
use std::time::Instant;
use crate::department::types::msg::{DognutOption, TransferMsg};
use crate::department::types::multi_sender::MultiSender;
use image::ImageEncoder;
use log::{debug, info};
use protobuf::Message;

extern crate turbojpeg;

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
                        //let mut out = Vec::new();
                        let img = image::RgbaImage::from_raw(self.dimension.0, self.dimension.1, data).unwrap();
                        let out = turbojpeg::compress_image(&img, 50, turbojpeg::Subsamp::Sub2x2 ).unwrap();

                        let mut vid_packet = crate::pb::avpacket::VideoPacket::new();
                        vid_packet.data_len = out.len() as u32;
                        vid_packet.data = out.to_vec();
                        debug!("encode image cost {:?} with size {}", instant.elapsed(), vid_packet.data_len);

                        let serialized = vid_packet.write_to_bytes().unwrap();
                        if self.ms.net.send(TransferMsg::CompressedData(serialized)).is_err() {
                            info!("img encoder thread quit on send err");
                            return;
                        }

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
