pub mod encode;

pub struct VConvert();

// const Kr:f32 = 0.2126;
// const Kb:f32 = 0.0722;
// const Kg:f32 = 1 - Kr - Kb;
//
// impl VConvert {
//     fn rgb2yuvAYUV(rgb: &[u8]) -> [u8;3] {
//         let (r, g, b) = (rgb[0] as f32, rgb[1] as f32, rgb[2] as f32);
//         let L = Kr * r + Kg * g + Kb * b;
//         let Y = ((219. * L) / 255. + 16. + 0.5).floor() as u8;
//         let U = ((112. * (b - L) / (( 1.- Kb) * 255.) + 128.) + 0.5).clamp(0., 255.) as u8;
//         let V = ((112. * (r - L) / (( 1.- Kr) * 255.) + 128.) + 0.5).clamp(0., 255.) as u8;
//
//         [Y, U, V]
//     }
// }