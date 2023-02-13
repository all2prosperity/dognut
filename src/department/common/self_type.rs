use crate::wgpu::camera as cg_camera;
use crate::wgpu::wgpu_helper;
use super::constant;
use crate::department::view::camera as dn_camera;
use crate::department::preview::vector;

pub type StateImp = wgpu_helper::State<cg_camera::Camera>;

pub fn camera_instance() -> cg_camera::Camera {
    // dn_camera::Camera::new(45., (constant::WIDTH / constant::HEIGHT) as f32,
    //     0.1, 100., vector::Vector3::from_xyz(0., 0., -10.,),
    //     vector::Vector3::from_xyz(0., 0., -1.),
    //     vector::Vector3::from_xyz(0., -1., 0.))
    let projection = cg_camera::Projection::new(
        constant::WIDTH, 
        constant::HEIGHT, 
        cgmath::Deg(45.), 
        0.1, 100.0);
    let camera = cg_camera::Camera::new((0.0, 0., 10.), cgmath::Deg(-90.0), cgmath::Deg(-0.0), projection);
    camera
}


