
use crate::wgpu::camera as cg_camera;
use crate::wgpu::wgpu_helper;



pub type StateImp = wgpu_helper::State<cg_camera::Camera>;

pub fn camera_instance(width: u32, height:u32) -> cg_camera::Camera {
    // dn_camera::Camera::new(45., (constant::WIDTH / constant::HEIGHT) as f32,
    //     0.1, 100., vector::Vector3::from_xyz(0., 0., -10.,),
    //     vector::Vector3::from_xyz(0., 0., -1.),
    //     vector::Vector3::from_xyz(0., -1., 0.))
    let projection = cg_camera::Projection::new(
        width ,
        height,
        cgmath::Deg(60.),
        0.1, 1000.0);
    let camera = cg_camera::Camera::new((0.0, 0., 10.), cgmath::Deg(-90.0), cgmath::Deg(-0.0), projection);
    camera
}
