use winit::event::VirtualKeyCode;
use crate::department::preview::position::Pos3;
use crate::department::preview::matrix::Matrix;
use crate::department::preview::object_buffer::ObjectBuffer;
use crate::department::preview::triangle::Triangle;
use crate::department::preview::output_buffer::OutputBuffer;
use crate::department::preview::vector::Vector3;

pub struct Camera {
    fov_y: f32,
    ratio: f32,
    n: f32,
    z: f32,
    pos: Pos3,
    forward: Vector3,
    up: Vector3,
}

type HMat = Matrix<4,4>;

impl Camera {
    pub fn new(fov_y: f32, ratio: f32, n: f32, z: f32, pos: Pos3, forward: Vector3, up: Vector3) -> Self{
        Self {
            fov_y,
            ratio,
            n,
            z,
            pos,
            forward,
            up,
        }
    }

    pub fn move_view(&mut self, input: VirtualKeyCode) {
        match input {
            VirtualKeyCode::Q => {
                let vec = self.up.cross(&self.forward);
                self.pos += vec;
            },
            VirtualKeyCode::E => {
                let vec = self.forward.cross(&self.up);
                self.pos += vec;
            },
            VirtualKeyCode::W => {
                self.pos += &self.forward;
            },
            VirtualKeyCode::S => {
                let vec = self.forward.cross(&self.up);
                self.pos -= &self.forward;
            },
            VirtualKeyCode::A => {
                let r = self.up.rotation_matrix(- std::f32::consts::PI / 180.);
                let vec = self.forward.cross(&self.up);
                self.forward = &self.forward * &r;
            },
            VirtualKeyCode::D => {
                let r = self.up.rotation_matrix(std::f32::consts::PI / 180.);
                let vec = self.forward.cross(&self.up);
                self.forward = &self.forward * &r;
            },
            _ => {},
        };
    }

    pub fn to_transform_matrix(&self) -> HMat {
        let fov_x = self.fov_y * self.ratio;
        let (n, f, l, r, b, t) = (self.n, self.z, -fov_x / 2., fov_x / 2., -self.fov_y / 2., self.fov_y / 2.);

        let persp = HMat::from_vec(
                         vec![
                            self.n, 0., 0., 0.,
                            0., self.n, 0., 0.,
                            0., 0., self.n + self.z, - self.n * self.z,
                            0., 0., 1., 0.,
                         ]);

        let ort1 = HMat::from_vec(vec![
                            2. / (r - l), 0., 0., 0.,
                            0., 2. / (t - b), 0., 0.,
                            0., 0., 2. / (n - f), 0.,
                            0., 0., 0., 1.,
                        ]);

        let ort2 = HMat::from_vec(vec![
                            1., 0., 0., -(r + l) / 2.,
                            0., 1., 0., -(t + b) / 2.,
                            0., 0., 1., -(n + f) / 2.,
                            0., 0., 0., 1.,
                        ]);
        ((ort1 * ort2) * persp)
    }

    pub fn to_view_matrix(&self) -> HMat{
        let t = HMat::from_vec(vec![
            1., 0., 0., -self.pos.x(),
            0., 1., 0., -self.pos.y(),
            0., 0., 1., -self.pos.z(),
            0., 0., 0., 1.,
        ]);
        let g_t = self.forward.cross(&self.up);

        let mut r = HMat::from_vec( vec![
            g_t.x(), self.up.x(), -self.forward.x(), 0.,
            g_t.y(), self.up.y(), -self.forward.y(), 0.,
            g_t.z(), self.up.z(), -self.forward.z(), 0.,
            0., 0., 0., 1.,
        ]);
        let rt = r.t();
        rt * t
    }

    #[profiling::function]
    //pub fn render(&self, width: u32, height: u32, object_buffer: &ObjectBuffer, view: &HMat) -> OutputBuffer {
    pub fn render(&self, width: u32, height: u32, object_buffer: &ObjectBuffer, model: &HMat) -> OutputBuffer {
        let mut _out = OutputBuffer::new(width, height);
        let projection = self.to_transform_matrix();
        let view = self.to_view_matrix();

        let mvp = &view * model;
        let mvp = &projection * &mvp;

        for _tri in object_buffer.iter() {
            let trans_poses = _tri.v.iter().map(|x| &mvp * &x.to_homogeneous());
            let trans_poses = trans_poses.map(|x| Pos3::from_matrix(&x));
            for pos in trans_poses.clone() {
                if pos.x() < -1. || pos.x() > 1. || pos.y() > 1. || pos.y() < -1.{
                    println!("will return: {:?}", pos);
                    return _out;
                }
            }

            let surface_tri_zero = Triangle::from_vec(
                trans_poses.clone().map(|x| _out.pos_to_pixel_pos(&x)).collect()
                );

            let surface_tri_tilt = Triangle::from_vec(
                trans_poses.map(|x| _out.pos_to_pixel_pos_with_z(&x)).collect()
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
                        let depth = (&depth_matrix * &pos.to_homogeneous()).result();
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
