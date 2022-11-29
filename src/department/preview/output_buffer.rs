use std::io::Stdout;
use std::path::Path;
use crossterm::{cursor, execute, queue, style};
use crossterm::style::Color;
use image;
use image::{ImageBuffer, ImageFormat, RgbaImage};
use crossterm::style::Stylize;
use log::error;
use super::position::Pos3;
use super::matrix::Matrix;
pub type Display = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;

pub struct OutputBuffer<'a> {
    width: u32,
    height: u32,
    pub display: Vec<u8>,
    pub depth: Vec<f32>,
    tui: bool,
    pub stdout: Option<&'a Stdout>,
}

const RGB_STEP: usize = 4;

impl<'a> OutputBuffer<'a> {
    pub fn new(width: u32, height: u32, tui: bool) -> Self {
        let pixels_num = (width * height) as usize;
        let mut _depth: Vec<f32> = Vec::with_capacity(pixels_num);
        _depth.resize(pixels_num, f32::NEG_INFINITY);

        let mut _display: Vec<u8> = Vec::with_capacity(pixels_num * RGB_STEP);
        if tui {
            _display.resize(pixels_num * RGB_STEP, 0);
        }

        Self {
            width, height,
            display: _display,
            depth: _depth,
            tui,
            stdout: None
        }
    }

    pub fn get_depth(&self, x: usize, y: usize) -> f32 {
        self.depth[y * self.width as usize + x]
    }

    pub fn set_depth(&mut self, x: usize, y: usize, val: f32) {
        self.depth[y * self.width as usize + x] = val;
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, rgb: &[u8]) {
        if self.tui {
            let mut stdout = self.stdout.unwrap();
            queue!(stdout, cursor::MoveTo(x as u16, y as u16));
            queue!(stdout, style::PrintStyledContent("•".with(Color::Rgb {r: rgb[0], g: rgb[1], b:rgb[2]})));
            return ;
        }
        let start = (y * self.width + x) as usize * RGB_STEP;
        let buf = &mut self.display[start..(start + RGB_STEP)];
        for i in 0..RGB_STEP {
            buf[i] = rgb[i];
        }
    }

    pub fn pos_to_pixel(&self, x: f32, y: f32) -> (f32, f32) {
        (self.width as f32 / 2. * (x + 1.), self.height as f32 / 2. * (1. - y))
    }

    pub fn to_view_port_matrix(&self) -> Matrix<4, 4>{
        let half_width = self.width as f32 / 2.;
        let half_height = self.height as f32 / 2.;
        Matrix::<4, 4>::from_vec(vec![
            half_width, 0., 0., 0.,
            0., -half_height, 0., 0.,
            0., 0., 1., 0.,
            half_width, half_height, 0., 1.,
        ])
    }

    pub fn pos_to_pixel_pos(&self, pos: &Pos3) -> Pos3{
        let (x, y) = (self.width as f32 / 2. * (pos.x() + 1.), self.height as f32 / 2. * (1. - pos.y()));
        Pos3::from_xyz(x, y, 0.)
    }

    pub fn pos_to_pixel_pos_with_z(&self, pos: &Pos3) -> Pos3{
        let (x, y) = (self.width as f32 / 2. * (pos.x() + 1.), self.height as f32 / 2. * (1. - pos.y()));
        Pos3::from_xyz(x,y,pos.z())
    }


    pub fn save_to_image(&self, path: &str) {
        let mut img = RgbaImage::new(self.width, self.height);
        img.copy_from_slice(self.display.as_slice());
        if let Err(e) = img.save_with_format(Path::new(path), ImageFormat::Png) {
            error!("could not save image {}", e);
        }
    }
}
