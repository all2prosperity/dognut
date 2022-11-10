use image;
use super::position::Pos3;
pub type Display = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;

pub struct OutputBuffer {
    width: u32,
    height: u32,
    pub display: Vec<u8>,
    depth: Vec<f32>,
}

const RGB_STEP: usize = 4;

impl OutputBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let pixels_num = (width * height) as usize;
        let mut _depth: Vec<f32> = Vec::with_capacity(pixels_num);
        _depth.resize(pixels_num, -2.);

        let mut _display: Vec<u8> = Vec::with_capacity(pixels_num * RGB_STEP);
        _display.resize(pixels_num * RGB_STEP, 0);

        Self {
            width, height,
            display: _display,
            depth: _depth,
        }
    }

    pub fn get_depth(&self, x: usize, y: usize) -> f32 {
        self.depth[y * self.width as usize + x]
    }

    pub fn set_depth(&mut self, x: usize, y: usize, val: f32) {
        self.depth[y * self.width as usize + x] = val;
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, rgb: &[u8]) {
        let start = (y * self.width + x) as usize * RGB_STEP;
        let buf = &mut self.display[start..(start + RGB_STEP)];
        for i in 0..RGB_STEP {
            buf[i] = rgb[i];
        }
    }

    pub fn pos_to_pixel(&self, x: f32, y: f32) -> (f32, f32) {
        (self.width as f32 / 2. * (x + 1.), self.height as f32 / 2. * (1. - y))
    }

    pub fn pos_to_pixel_pos(&self, pos: &Pos3) -> Pos3{
        let (x, y) = (self.width as f32 / 2. * (pos.x() + 1.), self.height as f32 / 2. * (1. - pos.y()));
        Pos3::from_xyz(x, y, 0.)
    }

    pub fn pos_to_pixel_pos_with_z(&self, pos: &Pos3) -> Pos3{
        let (x, y) = (self.width as f32 / 2. * (pos.x() + 1.), self.height as f32 / 2. * (1. - pos.y()));
        Pos3::from_xyz(x,y,pos.z())
    }
}
