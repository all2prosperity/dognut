use cgmath;

use dognut::department::common::constant;
use dognut::department::preview::{position, vector};
use dognut::department::view::camera;
use dognut::department::view::camera_trait::CameraTrait;
use dognut::wgpu::camera as cg_camera;

fn main() {
    let dn_camera = camera::Camera::new(
        45.,
        constant::WIDTH as f32 / constant::HEIGHT as f32,
        0.1,
        100.,
        position::Pos3::from_xyz(0.0, 0., 10.),
        vector::Vector3::from_xyz(0., 0., 1.),
        vector::Vector3::from_xyz(0., 1., 0.),
    );
    let view = dn_camera.to_view_matrix();
    view.debug();
    dn_camera.perspective_projection.debug();
    let proj_a: [[f32; 4]; 4] = dn_camera.perspective_projection.clone().into();
    println!("dn persp: {:?}", proj_a);

    let projection = cg_camera::Projection::new(
        constant::WIDTH, constant::HEIGHT, cgmath::Deg(45.), 0.1, 100.0);
    let cg_camera = cg_camera::Camera::new(
        (0.0, 0., 10.), 
        cgmath::Deg(-90.0), 
        cgmath::Deg(-0.0), 
        projection);

    let cg_persp:[[f32; 4]; 4] = cg_camera.proj.calc_matrix().into();
    println!("cg persp:{:?}", cg_camera.proj.calc_matrix());

    let proj = cg_camera.to_view_proj();
    println!("cg proj:{:?}", proj);

    let proj = dn_camera.to_view_proj();
    println!("dn proj:{:?}", proj);

    let view = cg_camera.calc_matrix();
    println!("cg view :{:?}", view);

    let view = dn_camera.to_view_matrix();
    view.debug();

}
