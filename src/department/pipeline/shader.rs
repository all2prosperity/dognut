// use crate::department::preview::triangle::Triangle;
// use crate::department::preview::vector::Vector3;
//
// #[derive(Debug)]
// pub struct LambertianShader {
//     light_source: Vector3,
//     kd: f32,
//     light_intensity: f32,
// }
//
// impl LambertianShader {
//     pub fn new(light_source:Vector3, kd: f32, light_intensity: f32) -> Self {
//         Self { light_source, kd, light_intensity }
//     }
//
//     pub fn luminance(&self,x:usize, y:usize, tri: &Triangle) -> f32 {
//         0.
//         // let mut l =  self.light_source.dot(&tri.get_normal(x, y));
//         // if l < 0. {
//         //     l = 0.;
//         // }
//
//         // tri.get_color(x, y) * l
//     }
// }