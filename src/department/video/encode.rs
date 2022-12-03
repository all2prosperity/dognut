use std::ffi::c_int;
use ffmpeg_next as ffmpeg;

use ffmpeg::encoder::video;
use ffmpeg::codec;
use ffmpeg::ffi;
use ffmpeg::software::scaling;
use ffmpeg_next::Codec;
use ffmpeg_next::codec::Context;

use ffmpeg_next::codec::Id::H264;
use ffmpeg_next::format::Pixel;
use ffmpeg_next::software::scaling::Flags;


pub struct x264Encoder {
    tx: crossbeam_channel::Sender<Vec<u8>>,
    encoder: video::Encoder,
    scale_ctx: scaling::Context,

    codec: Codec,
}


impl x264Encoder {
    pub unsafe fn new(tx: crossbeam_channel::Sender<Vec<u8>>, dimension: (u32, u32)) -> Result<Self, ffmpeg::Error> {
        ffmpeg::init()?;
        let codec = codec::encoder::find(H264).expect("can't find h264 encoder");

        let context = Self::wrap_context(&codec, dimension);

        let video = context.encoder().video()?;
        let encoder = video.open_as(codec)?;

        let scaler = scaling::Context::get(Pixel::RGBA, dimension.0, dimension.1,
        Pixel::YUV420P, dimension.0, dimension.1, Flags::BILINEAR)?;

        Ok(Self {
            tx,
            encoder,
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
        let mut k = std::ffi::CString::new("preset").unwrap();
        let mut v = std::ffi::CString::new("fast").unwrap();
        ffi::av_opt_set(raw_context as *mut _, k.as_ptr(), v.as_ptr(), 0);
        k = std::ffi::CString::new("x264-params").unwrap();
        v = std::ffi::CString::new("keyint=60:min-keyint=60:scenecut=0:force-cfr=1").unwrap();
        ffi::av_opt_set(raw_context as *mut _, k.as_ptr(), v.as_ptr(), 0);



        return Context::wrap(raw_context, None);
    }
}
