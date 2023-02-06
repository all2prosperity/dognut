use dognut::department::view::camera;
use dognut::department::preview::{position, vector};
use dognut::wgpu::camera as cg_camera;
use dognut::department::common::constant;
use dognut::department::view::camera_trait::CameraTrait;
use cgmath;


fn main() {
    let camera = camera::Camera::new(
        45.,
        constant::WIDTH as f32 / constant::HEIGHT as f32,
        0.1,
        1000.,
        position::Pos3::from_xyz(0.0, 0., -500.),
        vector::Vector3::from_xyz(0., 0., 1.),
        vector::Vector3::from_xyz(0., 1., 0.),
    );
    let view = camera.to_view_matrix();
    view.debug();
    camera.perspective_projection.debug();

    let projection = cg_camera::Projection::new(constant::WIDTH, constant::HEIGHT, cgmath::Deg(45.), 0.1, 100.0);
    let cg_camera = cg_camera::Camera::new((0.0, 0., 10.), cgmath::Deg(-90.0), cgmath::Deg(-0.0), projection);
    let proj = cg_camera.to_view_proj();
    println!("cg proj:{:?}", proj);
}
