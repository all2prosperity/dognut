use std::f32::consts::PI;
use winit::event::VirtualKeyCode;
use crate::department::preview::homo_transformation::{HomoTransform, Transform};
use crate::department::preview::position::Pos3;
use crate::department::preview::matrix::{HMat, Matrix};
use crate::department::preview::object_buffer::ObjectBuffer;
use crate::department::model::triangle::Triangle;
use crate::department::preview::output_buffer::OutputBuffer;
use crate::department::preview::vector::Vector3;

pub struct Camera {
    fov_y: f32,
    ratio: f32,
    n: f32,
    z: f32,
    eye: Pos3,
    forward: Vector3,
    up: Vector3,
    perspective_projection: HMat
}

impl Camera {
    pub fn new(fov_y: f32, ratio: f32, n: f32, z: f32, pos: Pos3, forward: Vector3, up: Vector3) -> Self{
        let persp =  Camera::perspective_projection_mat(fov_y, ratio, n, z);
        Self {
            fov_y,
            ratio,
            n,
            z,
            eye: pos,
            forward,
            up,
            perspective_projection: persp,
        }
    }

    pub fn move_view(&mut self, input: VirtualKeyCode) {
        match input {
            VirtualKeyCode::Q => {
                let vec = self.up.cross(&self.forward);
                self.eye += vec;
            },
            VirtualKeyCode::E => {
                let vec = self.forward.cross(&self.up);
                self.eye += vec;
            },
            VirtualKeyCode::W => {
                self.eye += &self.forward;
            },
            VirtualKeyCode::S => {
                let vec = self.forward.cross(&self.up);
                self.eye -= &self.forward;
            },
            VirtualKeyCode::A => {
                // let r = Transform::rotation_matrix(&self.up, -std::f32::consts::PI / 180.);
                // let vec = self.forward.cross(&self.up);
                //
                // self.forward = &self.forward * &r;
            },
            VirtualKeyCode::D => {
                // let r = Transform::rotation_matrix(&self.up, std::f32::consts::PI / 180.);
                // let vec = self.forward.cross(&self.up);
                // self.forward = &self.forward * &r;
            },
            _ => {},
        };
    }

    fn calc_lrtbnf(half_fov_y_degree: f32, ratio: f32, n: f32, f: f32) -> (f32, f32, f32, f32, f32, f32) {
        let tan_y = ((half_fov_y_degree / 180.) * PI).tan();
        let y = ((n * tan_y) * 2.).abs();
        let x = y * ratio;

        (-x/2., x/2., y/2., -y/2., n, f)
    }

    pub fn perspective_projection_mat(fov_y: f32, ratio: f32, n: f32, z: f32) -> HMat {
        // let tan_camera =  ((fov_y / 180.) * PI).tan();
        // let y = (n * tan_camera) * 2.;
        // let fov_x = fov_y * ratio;
        let (l, r, t, b, n, f) = Camera::calc_lrtbnf(fov_y / 2., ratio, n , z);

        let persp = HMat::from_vec(
                         vec![
                            n, 0., 0., 0.,
                            0., n, 0., 0.,
                            0., 0., n + f, 1.,
                            0., 0., -n * f, 0.,
                         ]);

        let ort_scale = HMat::from_vec(vec![
                            2. / (r - l), 0., 0., 0.,
                            0., 2. / (t - b), 0., 0.,
                            0., 0., 2. / (n - f), 0.,
                            0., 0., 0., 1.,
                        ]);

        let ort_translate = HMat::from_vec(vec![
                            1., 0., 0.,0.,
                            0., 1., 0., 0.,
                            0., 0., 1., 0.,
                            -(r + l) / 2., -(t + b) / 2., -(n + f) / 2., 1.,
                        ]);

        persp * ort_translate * ort_scale
    }

    pub fn to_view_matrix(&self) -> HMat{
        let mut fwd = self.forward.clone();
        fwd.norm();
        let w =  fwd * -1f32;
        let mut u = self.up.cross(&self.forward);
        u.norm();
        let v = w.cross(&u);
        let t = HMat::from_vec(vec![
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            -self.eye.x(), -self.eye.y(), -self.eye.z(), 1.,
        ]);
        //let g_t = self.forward.cross(&self.up);

        let r = HMat::from_vec( vec![
            u.x(), v.x(), w.x(), 0.,
            u.y(), v.y(), w.y(), 0.,
            u.z(), v.z(), w.z(), 0.,
            0., 0., 0., 1.,
        ]);

        t * r
    }

    #[profiling::function]
    //pub fn render(&self, width: u32, height: u32, object_buffer: &ObjectBuffer, view: &HMat) -> OutputBuffer {
    pub fn render(&self, width: u32, height: u32, object_buffer: &ObjectBuffer, model: &HMat) -> OutputBuffer {
        let mut _out = OutputBuffer::new(width, height);

        let view = self.to_view_matrix();

        let mvp = &(model * &view) * &self.perspective_projection;

        for _tri in object_buffer.iter() {
            let trans_poses = _tri.v.iter().map(|x| &x.to_homogeneous() * &mvp);
            let trans_poses:Vec<Pos3> = trans_poses.map(|x| Pos3::from_matrix(&x)).collect();
            for  pos in &trans_poses {
                if pos.x() < -1. || pos.x() > 1. || pos.y() > 1. || pos.y() < -1.{
                    println!("will return: {:?}", pos);
                    return _out;
                }
            }

            let surface_tri_zero = Triangle::from_vec(
                trans_poses.iter().map(|x| _out.pos_to_pixel_pos(&x)).collect()
                );

            let surface_tri_tilt = Triangle::from_vec(
                trans_poses.iter().map(|x| _out.pos_to_pixel_pos_with_z(&x)).collect()
                );

            // println!("tilt:{:?}", surface_tri_tilt);

            // println!("surface tri {:?}", surface_tri_tilt);

            let (sx, ex, sy, ey) = surface_tri_zero.get_edge();
            let depth_matrix = surface_tri_tilt.get_depth_matrix();
            // println!("edge :{:?}", (sx, ex, sy, ey));
            // let pos = Pos3::new(330., 420., 0.);
            // let ret = surface_tri_zero.in_triangle(&pos);
            // println!("ret is {:?}", ret);
            //
            for j in sy..ey {
                if let Some((_sx, _ex)) = surface_tri_zero.get_horizon_edge(j as f32 + 0.5, sx, ex) {
                    // println!("_sx:{:?}, {:?}", _sx, _ex);
                    for i in _sx..(_ex + 1) {
                        let pos = Pos3::from_xyz(i as f32 + 0.5, j as f32 + 0.5, 0.);
                        let depth = (&pos.to_homogeneous() * &depth_matrix).result();
                        let cur_depth = _out.get_depth(i as usize, j as usize);
                        if depth > cur_depth {
                            _out.set_depth(i as usize, j as usize, depth);
                            let color = (255 as f32 * (depth + 1.) / 2.).floor() as u8;
                            // println!("depth:{:?}, {:?}", depth, color);
                            _out.put_pixel(i, j, &[color, color, color, color]);
                        }
                    }
                }
            }

            // for i in sx..ex {
            //     for j in sy..ey {
            //         let pos = Pos3::new(i as f32 + 0.5, j as f32 + 0.5, 0.);
            //         if surface_tri_zero.in_triangle(&pos) {
            //             let depth = (&depth_matrix * &pos.to_matrix()).unwrap().result();
            //             let cur_depth = _out.get_depth(i as usize, j as usize);
            //             if depth > cur_depth {
            //                 _out.set_depth(i as usize, j as usize, depth);
            //                 let color = (255 as f32 * (depth + 1.) / 2.).floor() as u8;
            //                 // println!("depth:{:?}, {:?}", depth, color);
            //                 _out.put_pixel(i, j, &[color, color, color, color]);
            //             }
            //         }
            //     }
            // }
            // println!("edge2 :{:?}", (sx, ex, sy, ey));
        }

        _out
    }
}
