use std::cmp::max;
use crate::department::model::triangle::Triangle;
use crate::department::preview::vector::Vector3;

#[derive(Debug)]
pub struct LambertianShader {
    light_source: Vector3,
    kd: f32,
    light_intensity: f32,
}

impl LambertianShader {
    pub fn new(light_source:Vector3, kd: f32, light_intensity: f32) -> Self {
        Self { light_source, kd, light_intensity }
    }

    pub fn shade(&self,x:usize, y:usize, tri: &Triangle) -> Vector3 {
        let mut intensity =  self.light_source.dot(&tri.get_normal(x, y));
        if intensity < 0. {
            intensity = 0.;
        }
        let bar = tri.barycentric_2d((x as f32, y as f32));
        tri.get_color(&bar) * intensity
    }
}