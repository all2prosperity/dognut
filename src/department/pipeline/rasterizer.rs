use crate::department::preview::matrix::Matrix;
use crossbeam_channel::Sender;
use image::GenericImageView;
use crate::department::model::triangle::Triangle;
use crate::department::model::triangle_resources::TriangleResources;
use crate::department::pipeline::shader::Shader;
use crate::department::preview::homo_transformation::HomoTransform;
use crate::department::preview::output_buffer::OutputBuffer;
use crate::department::preview::vector::Vector3;
use crate::department::view::camera::Camera;


pub struct RasterRunner {
    tx: Sender<Box<Vec<u8>>>,
    model_mat: HomoTransform,
    view_mat: HomoTransform,
    proj_mat: HomoTransform,
    camera: Camera,
    shader: Box<dyn Shader>,
    tui: bool
}


impl RasterRunner {
    pub fn new(tx: Sender<Box<Vec<u8>>>, camera: Camera, shader: Box<dyn Shader>, tui: bool) -> Self {
        Self {
            tx,
            model_mat: HomoTransform::identity_matrix(),
            view_mat: camera.to_view_matrix(),
            proj_mat: camera.perspective_projection.clone(),
            camera,
            shader,
            tui,
        }
    }
    
    pub fn set_model(&mut self, m: HomoTransform) {
        self.model_mat = m;
    }


    pub fn render_frame(&self, triangle_res: &TriangleResources, out:&mut OutputBuffer) {
        let mv = &self.model_mat * &self.view_mat;
        let mvp = &mv * &self.proj_mat;
        let view_port = out.to_view_port_matrix();
        let image = triangle_res.image.as_ref().unwrap();

        for mut triangle in triangle_res.iter() {
            let screen = triangle.clip_return_screen_no_divide(&mvp, &view_port);
            let screen_divide: Vec<Vector3> = screen.iter().map(|v| {
                let d = v / v.index(0, 3);
                Vector3::from_xyz(d.index(0, 0), d.index(0, 1), d.index(0, 2))
            }).collect();

            let (sx, ex, sy, ey) = Triangle::bounding_box(&screen_divide);


            for i in sx..ex {
                for j in sy..ey {
                    let p = Vector3::from_xyz(i as f32 + 0.5, j as f32 + 0.5, 0.);

                    let bar = Triangle::barycentric_2d_out((p.x(), p.y()), &screen_divide);

                    if bar.x() < 0. || bar.y() < 0. || bar.z() < 0. {
                        continue;
                    }

                    let reci = 1. / (bar.x() / screen[0].w() + bar.y() / screen[1].w() + bar.z() / screen[2].w());
                    let bar_correct = Vector3::from_xyz(
                        (bar.x() / screen[0].w()) * reci,
                        (bar.y() / screen[1].w()) * reci,
                        (bar.z() / screen[2].w()) * reci,
                    );


                    let z_current = bar_correct.dot(&Vector3::from_xyz(screen_divide[0].z(),
                                                                       screen_divide[1].z(),
                                                                       screen_divide[2].z()));

                    if z_current > out.get_depth(p.x() as usize, p.y() as usize) {
                        out.set_depth(p.x() as usize, p.y() as usize, z_current);
                        let uv = triangle.get_uv(&bar_correct);
                        let color = image.get_pixel(uv.u() as u32, uv.v() as u32);
                        out.put_pixel(i, j, &color.0);
                    }
                }
            }
        }
    }
}