use image;
use super::position::Pos3;
pub type Display = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;

pub struct OutputBuffer {
    width: u32,
    height: u32,
    pub display: Display,
    depth: Vec<f32>,
}


impl OutputBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let mut _depth: Vec<f32> = Vec::with_capacity((width * height) as usize);
        _depth.resize((width * height) as usize, -2.);
        Self {
            width, height,
            display: image::ImageBuffer::new(width, height),
            depth: _depth,
        }
    }

    pub fn get_depth(&self, x: usize, y: usize) -> f32 {
        self.depth[x * self.width as usize + y]
    }

    pub fn set_depth(&mut self, x: usize, y: usize, val: f32) {
        self.depth[x * self.width as usize + y] = val;
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, rgb: image::Rgba<u8>) {
        self.display.put_pixel(x, y, rgb);
    }

    pub fn pos_to_pixel(&self, x: f32, y: f32) -> (f32, f32) {
        (self.width as f32 / 2. * (x + 1.), self.height as f32 / 2. * (1. - y))
    }

    pub fn pos_to_pixel_pos(&self, pos: &Pos3) -> Pos3{
        let (x, y) = (self.width as f32 / 2. * (pos.x + 1.), self.height as f32 / 2. * (1. - pos.y));
        Pos3 {
            x, y, z: 0.
        }
    }
}
