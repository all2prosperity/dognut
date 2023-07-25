use std::ffi::c_int;
use std::thread::JoinHandle;
use std::time::{Instant};

use crossbeam_channel::{unbounded};
use ffmpeg::codec;
use ffmpeg::encoder::video;
use ffmpeg::ffi;
use ffmpeg::software::scaling;
use ffmpeg_next as ffmpeg;
use ffmpeg_next::Codec;
use ffmpeg_next::codec::Context;
use ffmpeg_next::codec::Id::H264;
use ffmpeg_next::format::Pixel;
use ffmpeg_next::frame::Video;
use ffmpeg_next::log::Level;
use ffmpeg_next::picture::Type;
use ffmpeg_next::software::scaling::Flags;
use log::{error, info};
use protobuf::Message;



use crate::department::types::control::ControlMsg;
use crate::department::types::msg::{TransferMsg, DognutOption};
use crate::department::types::multi_sender::MultiSender;
use crate::pb;

pub struct RgbaEncoder {
    rx: crossbeam_channel::Receiver<TransferMsg>,
    ms: MultiSender<TransferMsg>,
    message_rx: crossbeam_channel::Receiver<ControlMsg>,
    encoder: video::Encoder,
    scale_ctx: scaling::Context,
    dimension: (u32, u32),
    codec: Codec,
    next_frame_idr: bool,
    instant: Instant,
}


impl RgbaEncoder {

    pub fn run(receiver: crossbeam_channel::Receiver<TransferMsg>, ms: MultiSender<TransferMsg>, dimension:(u32, u32)) -> JoinHandle<()>{
        ffmpeg::init().unwrap();
        ffmpeg::log::set_level(Level::Trace);
        let handle = std::thread::Builder::new().name("dognut_encoder_thread".into()).spawn(move || {
            let encoder = unsafe {Self::new(receiver, ms, dimension).expect("ffmpeg encoder init failed") };
            encoder.run_encoding_pipeline();
            return ();
        }).unwrap();

        return handle;
    }


    pub unsafe fn new(rx: crossbeam_channel::Receiver<TransferMsg>, ms: MultiSender<TransferMsg>,  dimension: (u32, u32)) -> Result<Self, ffmpeg::Error> {
        let mut codec = codec::encoder::find(H264).expect("can't find h264 encoder");
        if cfg!(target_os = "macos") {
            codec = codec::encoder::find_by_name("h264_videotoolbox").expect("can't find h264_videotoolbox encoder");
        }else if cfg!(target_os = "windows") {
            codec = codec::encoder::find_by_name("h264_mf").expect("can't find h264_nvenc encoder");
        }

        let context = Self::wrap_context(&codec, dimension);

        let video = context.encoder().video()?;
        let encoder = video.open_as(codec)?;

        let scaler = scaling::Context::get(Pixel::RGBA, dimension.0, dimension.1,
        Pixel::YUV420P, dimension.0, dimension.1, Flags::BILINEAR)?;

        let msg_rx = unbounded();

        Ok(Self {
            ms,
            rx,
            encoder,
            dimension,

            scale_ctx: scaler,
            message_rx: msg_rx.1,
            codec,
            next_frame_idr: true,
            instant: Instant::now(),
        })
    }

    unsafe fn wrap_context(codec: &Codec, dimension:(u32, u32)) -> Context {
        let raw_codec = codec.as_ptr();
        let raw_context = ffi::avcodec_alloc_context3(raw_codec);
        (*raw_context).width = dimension.0 as c_int;
        (*raw_context).height = dimension.1 as c_int;
        (*raw_context).pix_fmt = ffi::AVPixelFormat::AV_PIX_FMT_YUV420P;
        (*raw_context).time_base = ffi::AVRational{num: 1, den: 60};
        (*raw_context).bit_rate = 2 * 1000 * 1000;
        (*raw_context).rc_buffer_size = 4 * 1000 * 1000;
        (*raw_context).rc_max_rate = 2 * 1000 * 1000;
        (*raw_context).rc_min_rate = (2.5 * 1000. * 1000.) as i64;
        (*raw_context).framerate = ffi::AVRational{num:60, den: 1};
        (*raw_context).gop_size = 15;
        // disable b frame for realtime streaming
        (*raw_context).max_b_frames = 0;
        (*raw_context).has_b_frames = 0;
        let mut k = std::ffi::CString::new("preset").unwrap();
        let mut v = std::ffi::CString::new("fast").unwrap();
        ffi::av_opt_set((*raw_context).priv_data as *mut _, k.as_ptr(), v.as_ptr(), 0);
        k = std::ffi::CString::new("x264-params").unwrap();
        v = std::ffi::CString::new("keyint=60:min-keyint=60:scenecut=0:force-cfr=1").unwrap();
        //ffi::av_opt_set((*raw_context).priv_data as *mut _, k.as_ptr(), v.as_ptr(), 0);

        return Context::wrap(raw_context, None);
    }

    pub fn send_frame(&mut self, rgba: &[u8]) -> Result<(), ffmpeg::Error> {
        let rgb_frame =unsafe{self.unwrap_rgba_to_avframe(rgba)} ;
        let mut yuv = Video::empty();
        if let Err(e) = self.scale_ctx.run(&rgb_frame, &mut yuv) {
            error!("scale error {:?}", e);
        }

        if self.next_frame_idr {
            yuv.set_kind(Type::I);
            self.next_frame_idr = false;
            info!("next frame is IDR");
        }

        let ts = Instant::now().duration_since(self.instant).as_millis();

        yuv.set_pts(Some((ts * (1 / 60)) as i64));

        // todo: (liutong)  set frame pts
        self.encoder.send_frame(&yuv)?;

        //todo: encoder wait on another thread to recv encoded data and send to network;
        Ok(())
    }


    unsafe fn unwrap_rgba_to_avframe(&self, rgba: &[u8]) -> Video {
        let raw_frame =  ffi::av_frame_alloc();
        ffi::av_image_fill_arrays((*raw_frame).data.as_mut_ptr(), (*raw_frame).linesize.as_mut_ptr(),rgba.as_ptr(), ffi::AVPixelFormat::AV_PIX_FMT_RGBA,
                                  self.dimension.0 as c_int, self.dimension.1 as c_int, 1);

        let mut frame = Video::wrap(raw_frame);
        frame.set_format(Pixel::RGBA);
        frame.set_width(self.dimension.0);
        frame.set_height(self.dimension.1);
        frame
    }

    pub fn run_encoding_pipeline(mut self) {

        loop {
            let msg = self.rx.recv().unwrap();
            match msg {
                TransferMsg::RenderedData(_) => {}
                TransferMsg::DogOpt(code) => {
                    if code == DognutOption::StartEncode {
                        self.ms.win.send(TransferMsg::DogOpt(DognutOption::EncoderStarted)).expect("must send ok");
                        break;
                    }
                }
                TransferMsg::QuitThread => {
                    info!("encoder thread quit on msg");
                    return ;
                }
                TransferMsg::Test(_) => {}
                _ => {}
            }
            // if let Ok(TransferMsg::DogOpt(code)) = self.rx.recv() {
            //     if code == DognutOption::StartEncode {
            //         info!("start encoding");
            //         break;
            //     }
            // }
        }

        let mut packet = ffmpeg::Packet::empty();
        let mut index = 0;
        loop {
            if let Ok(msg) = self.rx.try_recv() {
                match msg {
                    TransferMsg::RenderedData(data) => {
                        self.send_frame(&data).expect("should send encoder ok");
                        if index == 0 {
                            self.instant = Instant::now();
                        }
                    }
                    TransferMsg::DogOpt(_) => {}
                    TransferMsg::QuitThread => {
                        break;
                    }
                    TransferMsg::Test(_) => {}
                    _ => {}
                }
            }

            while self.encoder.receive_packet(&mut packet).is_ok() {
                info!("received encoded video and send it to network index {}", index);
                index += 1;
                let mut vid_packet = pb::avpacket::VideoPacket::new();
                vid_packet.data = packet.data().unwrap().to_vec();
                vid_packet.data_len = vid_packet.data.len() as u32;
                vid_packet.dts = packet.dts().unwrap_or(0);
                vid_packet.pts = packet.pts().unwrap_or(0);
                vid_packet.duration = packet.duration();
                vid_packet.flags = 0;
                vid_packet.idr_frame = packet.is_key();

                let serialized = vid_packet.write_to_bytes().unwrap();
                if self.ms.net.send(TransferMsg::RenderedData(serialized)).is_err() {
                    break;
                }
            }
        }

        info!("encoder thread quit");
    }
}
