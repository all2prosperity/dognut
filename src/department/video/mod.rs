use std::thread::JoinHandle;
use std::time::Instant;
use crate::department::types::msg::{DognutOption, TransferMsg};
use crate::department::types::multi_sender::MultiSender;

use log::{info};
use protobuf::Message;

#[cfg(feature = "turbo")]
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

    pub fn run_encoding_pipeline(self) {
        loop {
            let msg = self.rx.recv().unwrap();
            match msg {
                TransferMsg::RenderedData(data) => {
                    drop(data);
                }
                TransferMsg::DogOpt(code) => {
                    if code == DognutOption::StartEncode {
                        self.ms.win.send(TransferMsg::DogOpt(DognutOption::EncoderStarted)).expect("must send ok");
                        break;
                    }
                }
                TransferMsg::QuitThread => {
                    info!("encoder thread quit on msg");
                    return;
                }
                _ => {}
            }
        }

        //self.ms.win.send(TransferMsg::DogOpt(DognutOption::EncoderStarted)).expect("must send ok");

        let _index = 0;
        loop {
            if let Ok(msg) = self.rx.recv() {
                match msg {
                    TransferMsg::RenderedData(data) => {
                        let instant = Instant::now();
                        let vid_packet = self.encode_vid_packet(data);
                        info!("encode image cost {:?} with size {}", instant.elapsed(), vid_packet.data_len);

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

    #[cfg(feature = "turbo")]
    fn encode_vid_packet(&self, data: Vec<u8>) -> crate::pb::avpacket::VideoPacket {
        let img = image::RgbaImage::from_raw(self.dimension.0, self.dimension.1, data).unwrap();
        let out = turbojpeg::compress_image(&img, 50, turbojpeg::Subsamp::Sub2x2).unwrap();
        let mut vid_packet = crate::pb::avpacket::VideoPacket::new();
        vid_packet.data_len = out.len() as u32;
        vid_packet.data = out.to_vec();
        vid_packet
    }

    #[cfg(not(feature = "turbo"))]
    fn encode_vid_packet(&self, data: Vec<u8>) -> crate::pb::avpacket::VideoPacket {
        let mut out = Vec::new();
        image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, 50)
            .encode(data.as_slice(), self.dimension.0, self.dimension.1, image::ColorType::Rgba8)
            .expect("encode image failed");

        let mut vid_packet = crate::pb::avpacket::VideoPacket::new();
        vid_packet.data_len = out.len() as u32;
        vid_packet.data = out;
        vid_packet
    }
}
