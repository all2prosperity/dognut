use dognut::department::view::camera;
use dognut::department::preview::{position, vector};
use cgmath;


struct Camera {
    eye: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Camera {
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        println!("view is {:?}, proj is {:?}", view, proj);
        proj * view
    }
}

fn main() {
    let width = 512;
    let height = 256;
    let camera = camera::Camera::new(
        45.,
        width as f32 / height as f32,
        0.1,
        1000.,
        position::Pos3::from_xyz(0.0, 0., -500.),
        vector::Vector3::from_xyz(0., 0., 1.),
        vector::Vector3::from_xyz(0., 1., 0.),
    );
    let view = camera.to_view_matrix();
    view.debug();

    let cg_camera = Camera {
        eye: (0.0, 0.0, -500.0).into(),
        target: (0.0, 0.0, 0.0).into(),
        up: cgmath::Vector3::unit_y(),
        aspect: width as f32 / height as f32,
        fovy: 45.0,
        znear: 0.1,
        zfar: 1000.0,
    };

    cg_camera.build_view_projection_matrix();
}
