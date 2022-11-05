use crate::department::preview::position::Pos3;
use crate::department::preview::matrix::Matrix;
use crate::department::preview::object_buffer::{ObjectBuffer, Triangle};
use image::{Rgba, ImageBuffer};
use crate::department::preview::output_buffer::{OutputBuffer};

pub struct Camera {
    fov_y: f32,
    ratio: f32,
    n: f32,
    z: f32,
}

impl Camera {
    pub fn new(fov_y: f32, ratio: f32, n: f32, z: f32) -> Self{
        Self {
            fov_y,
            ratio,
            n,
            z,
        }
    }

    pub fn to_transform_matrix(&self) -> Matrix {
        let fov_x = self.fov_y * self.ratio;
        let (n, f, l, r, b, t) = (self.n, self.z, -fov_x / 2., fov_x / 2., -self.fov_y / 2., self.fov_y / 2.); 

        let persp = Matrix::from_vec(4, 4, false, 
                         vec![
                            self.n, 0., 0., 0.,
                            0., self.n, 0., 0.,
                            0., 0., self.n + self.z, - self.n * self.z,
                            0., 0., 1., 0.,
                         ]).unwrap();

        let ort1 = Matrix::from_vec(4, 4, false,
                        vec![
                            2. / (r - l), 0., 0., 0.,
                            0., 2. / (t - b), 0., 0.,
                            0., 0., 2. / (n - f), 0.,
                            0., 0., 0., 1.,
                        ]).unwrap();

        let ort2 = Matrix::from_vec(4, 4, false, 
                        vec![
                            1., 0., 0., -(r + l) / 2.,
                            0., 1., 0., -(t + b) / 2.,
                            0., 0., 1., -(n + f) / 2.,
                            0., 0., 0., 1.,
                        ]).unwrap();

        ((ort1 * ort2).unwrap() * persp).unwrap() 
    }

    pub fn render(&self, width: u32, height: u32, object_buffer: &ObjectBuffer) -> OutputBuffer {
        let mut _out = OutputBuffer::new(width, height);
        let transform_matrix = self.to_transform_matrix();

        for _tri in object_buffer.buffer.iter() {
            let trans_poses = _tri.poses.iter().map(|x| (&transform_matrix * &(x.to_matrix())).unwrap());
            println!("trans matrix:{:?}", trans_poses);
            let trans_poses = trans_poses.map(|x| Pos3::from_matrix(x));
            let test: Vec<Pos3> = trans_poses.clone().collect();
            println!("trans pos:{:?}", test);
            let surface_poses = trans_poses.map(|x| _out.pos_to_pixel_pos(&x));
            let surface_tri = Triangle::from_vec(surface_poses.collect());
            // let a = (p3.y - p1.y) * (p3.z - p1.z) - (p2.z - p1.z) * (p3.y - p1.y);
            //
            // let b = (p3.x - p1.x) * (p2.z - p1.z) - (p2.x - p1.x) * (p3.z - p1.z);
            //
            // let c = (p2.x - p1.x) * (p3.y - p1.y) - (p3.x - p1.x) * (p2.y - p1.y);
            //
            // let d =  -(a * p1.x + b * p1.y + c * p1.z);

            println!("surface tri {:?}", surface_tri);

            let (sx, ex, sy, ey) = surface_tri.get_edge();
            println!("edge :{:?}", (sx, ex, sy, ey));
            // let pos = Pos3::new(330., 420., 0.);
            // let ret = surface_tri.in_triangle(&pos);
            // println!("ret is {:?}", ret);
            for i in sx..ex {
                for j in sy..ey {
                    let pos = Pos3::new(i as f32 + 0.5, j as f32 + 0.5, 0.);
                    if surface_tri.in_triangle(&pos) {
                        _out.put_pixel(i, j, Rgba([100, 100, 100, 100]));
                    }
                }
            }
            println!("edge2 :{:?}", (sx, ex, sy, ey));
        }

        _out   
    }
}
