use std::f32::consts::FRAC_PI_2;
use std::time::Duration;


use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use winit::dpi::PhysicalPosition;
use winit::event::*;

use crate::department::view::camera_trait;


use super::ModelController;

const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;

#[derive(Debug)]
pub struct CameraController {
    amount_left: f32,
    amount_right: f32,
    amount_forward: f32,
    amount_backward: f32,
    amount_up: f32,
    amount_down: f32,
    rotate_horizontal: f32,
    rotate_vertical: f32,
    scroll: f32,
    speed: f32,
    sensitivity: f32,
    pub ctrl_pressed: bool,
    pub model_ctrl: ModelController,
    tui:bool,
}

impl CameraController {
    pub fn new(speed: f32, sensitivity: f32, tui: bool) -> Self {
        Self {
            amount_left: 0.0,
            amount_right: 0.0,
            amount_forward: 0.0,
            amount_backward: 0.0,
            amount_up: 0.0,
            amount_down: 0.0,
            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,
            scroll: 0.0,
            speed,
            sensitivity,
            ctrl_pressed: false,
            model_ctrl: ModelController::new(speed, tui),
            tui
        }
    }

    pub fn process_tui_keyboard(&mut self, key: &KeyEvent) -> bool {
        if key.modifiers == KeyModifiers::CONTROL{
            if key.kind == KeyEventKind::Press {
                self.model_ctrl.process_keyboard_tui(key);
            }
            return true;
        }
        let amount = if key.kind == KeyEventKind::Press {1.0} else {0.0};
        match key.code {
            KeyCode::Backspace => {}
            KeyCode::Left | KeyCode::Char('a') => {
                self.amount_left += amount;
            }
            KeyCode::Right | KeyCode::Char('d') => {
                self.amount_right += amount;
            }
            KeyCode::Up | KeyCode::Char('w') => {
                self.amount_forward += amount;
            }
            KeyCode::Down | KeyCode::Char('s') => {
                self.amount_backward += amount;
            }
            KeyCode::Char('q') | KeyCode::Esc => {
                return false;
            }
            KeyCode::Char('x') => {
                self.amount_up += amount;
            }
            KeyCode::Char('z') => {
                self.amount_down += amount;
            }
            _ => {}
        }

        return true;
    }

    pub fn process_keyboard(&mut self, key: VirtualKeyCode, state: ElementState) -> bool{
        if self.ctrl_pressed {
            return self.model_ctrl.process_keyboard(key, state);
        }

        let amount = if state == ElementState::Pressed { 1.0 } else { 0.0 };
        match key {
            VirtualKeyCode::W | VirtualKeyCode::Up => {
                self.amount_forward = amount;
                true
            }
            VirtualKeyCode::S | VirtualKeyCode::Down => {
                self.amount_backward = amount;
                true
            }
            VirtualKeyCode::A | VirtualKeyCode::Left => {
                self.amount_left = amount;
                true
            }
            VirtualKeyCode::D | VirtualKeyCode::Right => {
                self.amount_right = amount;
                true
            }
            VirtualKeyCode::Space => {
                self.amount_up = amount;
                true
            }
            VirtualKeyCode::LShift => {
                self.amount_down = amount;
                true
            }

            VirtualKeyCode::Q => {
                false
            }
            _ => true,
        }
    }

    pub fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.rotate_horizontal = mouse_dx as f32;
        self.rotate_vertical = mouse_dy as f32;
    }

    pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
        self.scroll = -match delta {
            // I'm assuming a line is about 100 pixels
            MouseScrollDelta::LineDelta(_, scroll) => scroll * 100.0,
            MouseScrollDelta::PixelDelta(PhysicalPosition {
                                             y: scroll,
                                             ..
                                         }) => *scroll as f32,
        };
    }

    pub fn update_camera<T: camera_trait::CameraTrait>(&mut self, camera: &mut T, dt: Duration) {
        let dt = dt.as_secs_f32();

        // Move forward/backward and left/right
        let forward_dt = (self.amount_forward - self.amount_backward) * self.speed * dt;
        let right_dt = (self.amount_right - self.amount_left) * self.speed * dt;
        let scroll_dt = self.scroll * self.speed * self.sensitivity * dt;
        let up_dt = (self.amount_up - self.amount_down) * self.speed * dt;
        camera.update_camera(forward_dt, right_dt, scroll_dt, up_dt, self.rotate_horizontal, self.rotate_vertical, self.sensitivity * dt);
        self.scroll = 0.;
        self.rotate_horizontal = 0.0;
        self.rotate_vertical = 0.0;


        if self.tui {
            self.amount_backward = 0.;
            self.amount_down = 0.;
            self.amount_forward = 0.;
            self.amount_left =0.;
            self.amount_right =0.;
            self.amount_up = 0.;
        }


        // Move in/out (aka. "zoom")
        // Note: this isn't an actual zoom. The camera's position
        // changes when zooming. I've added this to make it easier
        // to get closer to an object you want to focus on.

        // Move up/down. Since we don't use roll, we can just
        // modify the y coordinate directly.

        // Rotate

        // If process_mouse isn't called every frame, these values
        // will not get set to zero, and the camera will rotate
        // when moving in a non cardinal direction.

        // Keep the camera's angle from going too high/low.
        // let dt = dt.as_secs_f32();
        //
        // // Move forward/backward and left/right
        // let (yaw_sin, yaw_cos) = camera.yaw.0.sin_cos();
        // let forward = Vector3::new(yaw_cos, 0.0, yaw_sin).normalize();
        // let right = Vector3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        // camera.position += forward * (self.amount_forward - self.amount_backward) * self.speed * dt;
        // camera.position += right * (self.amount_right - self.amount_left) * self.speed * dt;
        //

    }
}
