

use crate::department::preview::homo_transformation::HomoTransform;
use crate::department::preview::matrix::Matrix;
use crate::department::preview::vector::Vector3;
use crate::department::view::camera::Camera;

pub static LUMINANCE_CHARS: [char; 12] = ['.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@'];

//#[derive(Debug)]
pub struct LambertianShader {
    model_view: HomoTransform,
    model_view_IT: HomoTransform,
    light_source: Vector3,
    ka: f32,
    ks_index: f32,
    light_intensity: f32,
    tui: bool,
}

pub trait Shader {
    fn shade(&self, normal: &Vec<Vector3>, diffuse: &[u8; 4], bar: &Vector3) -> [u8;4];
}

impl LambertianShader{
    pub fn new(light_source: Vector3, ka: f32, light_intensity: f32, cam: &Camera, tui: bool) -> Self {
        let mv = &cam.model * &cam.to_view_matrix();
        let mut mv_it = HomoTransform::identity_matrix();
        if let Some(inverse) = mv.inverse_matrix() {
            mv_it = inverse.t();
        }

        let ls = &light_source.to_homogeneous() * &mv;
        let mut ls = Vector3::from_xyz(ls.x() / ls.w(), ls.y() / ls.w(), ls.z() / ls.w());
        ls.norm();
        ls *= -1.0;
        Self {
            light_source: ls,
            ka,
            light_intensity,
            model_view: mv,
            model_view_IT: mv_it,
            ks_index: 10.,
            tui,
        }
    }
}

impl Shader for LambertianShader {
    fn shade(&self, normal: &Vec<Vector3>, diffuse: &[u8;4], bar: &Vector3) -> [u8;4] {
        let mut n = Vec::new();
        for i in 0..normal.len() {
            let mut nn = Vector3::from_matrix(&(&normal[i].to_homogeneous() * &self.model_view_IT));
            nn.norm();
            n.push(nn);
        }
        let mut nl = bar * &Matrix::<3,3>::from_rows(n);
        nl.norm();
        let cos = nl.dot(&self.light_source);
        let intensity:f32 = if cos.le(&0.) {
            0.
        }else if cos.ge(&1.) {
            1.
        }else {
            cos
        };
        let index = ((LUMINANCE_CHARS.len() - 1) as f32 * intensity).ceil() as usize;
        let final_char = LUMINANCE_CHARS[index];

        let (r, g, b) = (intensity * diffuse[0] as f32, intensity * diffuse[1] as f32, intensity* diffuse[2] as f32);


        if self.tui {
            [r as u8 , g as u8, b as u8, final_char as u8]
        }else {
            [r as u8 , g as u8, b as u8, diffuse[3]]
        }


    }
}