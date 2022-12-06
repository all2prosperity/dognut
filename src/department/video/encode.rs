use std::ffi::c_int;
use crossbeam_channel::{RecvError, select};
use ffmpeg_next as ffmpeg;

use ffmpeg::encoder::video;
use ffmpeg::codec;
use ffmpeg::ffi;
use ffmpeg::software::scaling;
use ffmpeg_next::{Codec, Error};
use ffmpeg_next::codec::Context;

use ffmpeg_next::codec::Id::H264;
use ffmpeg_next::ffi::{AVFrame};
use ffmpeg_next::format::Pixel;
use ffmpeg_next::frame::Video;
use ffmpeg_next::software::scaling::Flags;
use log::error;

pub struct rgbaEncoder {
    rx: crossbeam_channel::Receiver<Box<[u8]>>,
    tx: crossbeam_channel::Sender<Vec<u8>>,
    encoder: video::Encoder,
    scale_ctx: scaling::Context,
    dimension: (u32, u32),
    codec: Codec,
}


impl rgbaEncoder {

    pub fn run(rgb_rx: crossbeam_channel::Receiver<Box<[u8]>>, network_tx: crossbeam_channel::Sender<Vec<u8>>, dimension:(u32, u32)) {
        std::thread::spawn(move || {
            let encoder = unsafe {Self::new(network_tx, rgb_rx, dimension).expect("ffmpeg encoder init failed") };
            encoder.run_decoding_pipeline();
        });
    }


    pub unsafe fn new(tx: crossbeam_channel::Sender<Vec<u8>>, rx: crossbeam_channel::Receiver<Box<[u8]>>,  dimension: (u32, u32)) -> Result<Self, ffmpeg::Error> {
        ffmpeg::init()?;
        let codec = codec::encoder::find(H264).expect("can't find h264 encoder");

        let context = Self::wrap_context(&codec, dimension);

        let video = context.encoder().video()?;
        let encoder = video.open_as(codec)?;

        let scaler = scaling::Context::get(Pixel::RGBA, dimension.0, dimension.1,
        Pixel::YUV420P, dimension.0, dimension.1, Flags::BILINEAR)?;

        Ok(Self {
            tx,
            rx,
            encoder,
            dimension,
            scale_ctx: scaler,
            codec
        })
    }

    unsafe fn wrap_context(codec: &Codec, dimension:(u32, u32)) -> Context {
        let raw_codec = codec.as_ptr();
        let raw_context = ffi::avcodec_alloc_context3(raw_codec);
        (*raw_context).width = dimension.0 as c_int;
        (*raw_context).height = dimension.1 as c_int;
        (*raw_context).pix_fmt = ffi::AVPixelFormat::AV_PIX_FMT_YUV420P;
        (*raw_context).time_base = ffi::AVRational{num: 1, den: 60};
        (*raw_context).bit_rate = 4 * 1000 * 1000;
        (*raw_context).rc_buffer_size = 8 * 1000 * 1000;
        (*raw_context).rc_max_rate = 10 * 1000 * 1000;
        (*raw_context).rc_min_rate = 2 * 1000 * 1000;
        (*raw_context).framerate = ffi::AVRational{num:60, den: 1};
        // disable b frame for realtime streaming
        (*raw_context).max_b_frames = 0;
        (*raw_context).has_b_frames = 0;
        let mut k = std::ffi::CString::new("preset").unwrap();
        let mut v = std::ffi::CString::new("fast").unwrap();
        ffi::av_opt_set(raw_context as *mut _, k.as_ptr(), v.as_ptr(), 0);
        k = std::ffi::CString::new("x264-params").unwrap();
        v = std::ffi::CString::new("keyint=60:min-keyint=60:scenecut=0:force-cfr=1").unwrap();
        ffi::av_opt_set(raw_context as *mut _, k.as_ptr(), v.as_ptr(), 0);

        return Context::wrap(raw_context, None);
    }

    pub fn send_packets(&mut self, rgba: &[u8]) -> Result<(), ffmpeg::Error> {
        let rgb_frame =unsafe{self.unwrap_rgba_to_avframe(rgba)} ;
        let mut yuv = Video::empty();
        self.scale_ctx.run(&rgb_frame, &mut yuv)?;

        self.encoder.send_frame(&yuv)?;

        //todo: encoder wait on another thread to recv encoded data and send to network;
        Ok(())
    }


    unsafe fn unwrap_rgba_to_avframe(&self, rgba: &[u8]) -> Video {
        let mut raw_frame =  ffi::av_frame_alloc();
        ffi::avpicture_fill(raw_frame as *mut ffi::AVPicture, rgba.clone().as_ptr(), ffi::AVPixelFormat::AV_PIX_FMT_RGBA,
        self.dimension.0 as c_int, self.dimension.1 as c_int);

        Video::wrap(raw_frame)
    }

    pub fn run_decoding_pipeline(mut self) {

        let mut packet = ffmpeg::Packet::empty();
        loop {
            let data = match self.rx.recv() {
                Ok(data) => {
                    data
                }
                Err(err) => {
                    error!("frame buffer data recv error {:?}", err.to_string());
                    break;
                }
            };

            self.send_packets(&data).expect("must send ok");
            while self.encoder.receive_packet(&mut packet).is_ok() {

            }

        }
    }
}
