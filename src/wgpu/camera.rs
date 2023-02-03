use cgmath::*;
use winit::event::*;
use winit::dpi::PhysicalPosition;
use std::time::Duration;
use std::f32::consts::FRAC_PI_2;
use crate::department::control::ModelController;
use crate::department::view::camera_trait;


#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;


#[derive(Debug)]
pub struct Camera {
    pub position: Point3<f32>,
    pub(crate) yaw: Rad<f32>,
    pub(crate) pitch: Rad<f32>,
    proj: Projection,
}

impl camera_trait::CameraTrait for Camera {
    fn update_camera(&mut self, forward_dt: f32, right_dt: f32, scroll_dt: f32, up_dt: f32, hori: f32, ver: f32, sensi: f32) {
        let (yaw_sin, yaw_cos) = self.yaw.0.sin_cos();
        let forward = Vector3::new(yaw_cos, 0.0, yaw_sin).normalize();
        let right = Vector3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        self.position += forward * forward_dt;
        self.position += right * right_dt;

        // Move in/out (aka. "zoom")
        // Note: this isn't an actual zoom. The camera's position
        // changes when zooming. I've added this to make it easier
        // to get closer to an object you want to focus on.
        let (pitch_sin, pitch_cos) = self.pitch.0.sin_cos();
        let scrollward = Vector3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin).normalize();
        self.position += scrollward * scroll_dt;

        // Move up/down. Since we don't use roll, we can just
        // modify the y coordinate directly.
        self.position.y += up_dt;

        // Rotate
        self.yaw += Rad(hori) * sensi;
        self.pitch += Rad(-ver) * sensi;

        // If process_mouse isn't called every frame, these values
        // will not get set to zero, and the camera will rotate
        // when moving in a non cardinal direction.

        // Keep the camera's angle from going too high/low.
        if self.pitch < -Rad(SAFE_FRAC_PI_2) {
            self.pitch = -Rad(SAFE_FRAC_PI_2);
        } else if self.pitch > Rad(SAFE_FRAC_PI_2) {
            self.pitch = Rad(SAFE_FRAC_PI_2);
        }
    }

    fn to_view_position(&self) -> [f32; 4] {
        self.position.to_homogeneous().into()
    }

    fn to_view_proj(&self) -> [[f32; 4]; 4] {
        (self.proj.calc_matrix() * self.calc_matrix()).into()
    }
}

impl Camera {
    pub fn new<
        V: Into<Point3<f32>>,
        Y: Into<Rad<f32>>,
        P: Into<Rad<f32>>,
    >(
        position: V,
        yaw: Y,
        pitch: P,
        proj: Projection,
    ) -> Self {
        Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
            proj: proj,
        }
    }

    pub fn update_proj(&mut self, proj: Projection) {
        self.proj = proj;
    }

    pub fn calc_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_to_rh(
            self.position,
            Vector3::new(
                self.yaw.0.cos(),
                self.pitch.0.sin(),
                self.yaw.0.sin(),
            ).normalize(),
            Vector3::unit_y(),
        )
    }
}

#[derive(Debug)]
pub struct Projection {
    aspect: f32,
    fovy: Rad<f32>,
    znear: f32,
    zfar: f32,
}

impl Projection {
    pub fn new<F: Into<Rad<f32>>>(
        width: u32,
        height: u32,
        fovy: F,
        znear: f32,
        zfar: f32,
    ) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy: fovy.into(),
            znear,
            zfar,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn calc_matrix(&self) -> Matrix4<f32> {
        OPENGL_TO_WGPU_MATRIX * perspective(self.fovy, self.aspect, self.znear, self.zfar)
    }
}

// #[derive(Debug)]
// pub struct CameraController {
//     amount_left: f32,
//     amount_right: f32,
//     amount_forward: f32,
//     amount_backward: f32,
//     amount_up: f32,
//     amount_down: f32,
//     rotate_horizontal: f32,
//     rotate_vertical: f32,
//     scroll: f32,
//     speed: f32,
//     sensitivity: f32,
//     pub ctrl_pressed: bool,
//     pub model_ctrl: ModelController,
// }
//
// impl CameraController {
//     pub fn new(speed: f32, sensitivity: f32) -> Self {
//         Self {
//             amount_left: 0.0,
//             amount_right: 0.0,
//             amount_forward: 0.0,
//             amount_backward: 0.0,
//             amount_up: 0.0,
//             amount_down: 0.0,
//             rotate_horizontal: 0.0,
//             rotate_vertical: 0.0,
//             scroll: 0.0,
//             speed,
//             sensitivity,
//             ctrl_pressed: false,
//             model_ctrl: ModelController::new(speed, false),
//         }
//     }
//
//     pub fn process_keyboard(&mut self, key: VirtualKeyCode, state: ElementState) -> bool{
//         if self.ctrl_pressed {
//             return self.model_ctrl.process_keyboard(key, state);
//         }
//
//         let amount = if state == ElementState::Pressed { 1.0 } else { 0.0 };
//         match key {
//             VirtualKeyCode::W | VirtualKeyCode::Up => {
//                 self.amount_forward = amount;
//                 true
//             }
//             VirtualKeyCode::S | VirtualKeyCode::Down => {
//                 self.amount_backward = amount;
//                 true
//             }
//             VirtualKeyCode::A | VirtualKeyCode::Left => {
//                 self.amount_left = amount;
//                 true
//             }
//             VirtualKeyCode::D | VirtualKeyCode::Right => {
//                 self.amount_right = amount;
//                 true
//             }
//             VirtualKeyCode::Space => {
//                 self.amount_up = amount;
//                 true
//             }
//             VirtualKeyCode::LShift => {
//                 self.amount_down = amount;
//                 true
//             }
//             _ => false,
//         }
//     }
//
//     pub fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
//         self.rotate_horizontal = mouse_dx as f32;
//         self.rotate_vertical = mouse_dy as f32;
//     }
//
//     pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
//         self.scroll = -match delta {
//             // I'm assuming a line is about 100 pixels
//             MouseScrollDelta::LineDelta(_, scroll) => scroll * 100.0,
//             MouseScrollDelta::PixelDelta(PhysicalPosition {
//                                              y: scroll,
//                                              ..
//                                          }) => *scroll as f32,
//         };
//     }
//
//     pub fn update_camera<T: camera_trait::CameraTrait>(&mut self, camera: &mut T, dt: Duration) {
//         let dt = dt.as_secs_f32();
//
//         // Move forward/backward and left/right
//         let forward_dt = (self.amount_forward - self.amount_backward) * self.speed * dt;
//         let right_dt = (self.amount_right - self.amount_left) * self.speed * dt;
//         let scroll_dt = self.scroll * self.speed * self.sensitivity * dt;
//         let up_dt = (self.amount_up - self.amount_down) * self.speed * dt;
//         camera.update_camera(forward_dt, right_dt, scroll_dt, up_dt, self.rotate_horizontal, self.rotate_vertical, self.sensitivity * dt);
//         self.scroll = 0.;
//         self.rotate_horizontal = 0.0;
//         self.rotate_vertical = 0.0;
//
//         // Move in/out (aka. "zoom")
//         // Note: this isn't an actual zoom. The camera's position
//         // changes when zooming. I've added this to make it easier
//         // to get closer to an object you want to focus on.
//
//         // Move up/down. Since we don't use roll, we can just
//         // modify the y coordinate directly.
//
//         // Rotate
//
//         // If process_mouse isn't called every frame, these values
//         // will not get set to zero, and the camera will rotate
//         // when moving in a non cardinal direction.
//
//         // Keep the camera's angle from going too high/low.
//     }
// }