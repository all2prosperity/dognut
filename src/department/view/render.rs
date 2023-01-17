#![deny(clippy::all)]
#![forbid(unsafe_code)]

use crate::department::preview::homo_transformation::{HomoTransform, Transform};
use crate::department::view::camera::Camera;

use crate::department::preview::matrix::HMat;
use crate::department::preview::position::Pos3;
use crate::department::preview::vector::Vector3;

use crate::department::common::constant::{HEIGHT, WIDTH};
use crate::department::model::object_loader::ObjectLoader;
use crate::department::model::triangle_resources::TriangleResources;
use crate::department::types::msg::TransferMsg;

use crossbeam_channel::Sender;

/// Representation of the application state. In this example, a box will bounce around the screen.
struct Render {
    cameras: Vec<Camera>,
    resources: TriangleResources,
    theta: f32,
}

impl Render {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        let res = ObjectLoader::load_triangle_resources("./res/Link/link_adult.obj");

        let mut cameras = Vec::new();
        cameras.push(Camera::new(
            45.,
            WIDTH as f32 / HEIGHT as f32,
            -5.,
            -50.,
            Pos3::from_xyz(0., 0., 10.),
            Vector3::from_xyz(0., 0., -1.),
            Vector3::from_xyz(0., -1., 0.),
        ));

        Self {
            cameras: cameras,
            resources: res,
            theta: 0.,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        self.theta += 0.02;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&mut self) -> Vec<u8> {
        // link_adult is too big
        let scale = HomoTransform::scale((0.05, 0.05, 0.05));
        // let mut scale = HomoTransform::identity_matrix();

        let _move_origin = HomoTransform::translation((-0., -0., 0.));
        let rotate = Transform::rotation_mat(&Vector3::from_xyz(0., 1., 0.), self.theta);
        // let _move_back = HomoTransform::translation((0., 0., -0.0));
        // let _mat = _move_origin * rotate * _move_back;
        // let _mat = HomoTransform::identity_matrix();
        let _mat = scale * rotate;

        let _move = HMat::from_vec(vec![
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., -3., -5.0, 1.,
        ]);

        //let _mat = _mat * scale;
        let _mat = _mat * _move;

        let _buf = self.cameras[0].render_triangle_obejct(WIDTH, HEIGHT, &self.resources, &_mat);

        _buf.display
    }
}

pub fn run(render_pc_s: Sender<TransferMsg>, render_cli_s: Sender<TransferMsg>) {
    let mut render = Render::new();
    loop {
        render.update();
        let buf = render.draw();
        render_pc_s.send(TransferMsg::RenderPc(buf.clone()));
        render_cli_s.send(TransferMsg::RenderPc(buf));
    }
}
