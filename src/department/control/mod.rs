use std::f32::consts::PI;

use cgmath::{InnerSpace, Rotation3};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use winit::event::{ElementState, VirtualKeyCode};


use crate::wgpu::instance::Instance;

pub mod camera_controller;

#[derive(Debug)]
pub struct ModelController {
    position: cgmath::Vector3<f32>,
    rotation: cgmath::Quaternion<f32>,
    amount_left: f32,
    amount_right: f32,
    amount_forward: f32,
    amount_backward: f32,
    amount_up: f32,
    amount_down: f32,
    presentation_mode: bool,
    theta: f32,
    rotate_horizontal: f32,
    rotate_vertical: f32,
    scroll: f32,
    speed: f32,
    tui: bool
}

impl ModelController {
    pub fn new(speed: f32, tui: bool) -> Self {
        let p = cgmath::Vector3{x: 0.0, y: 0., z:1.};
        let q = cgmath::Quaternion::from_axis_angle(p.normalize(), cgmath::Deg(0.0));
        Self {position: p,
            theta: 0.,
            rotation: q,
            presentation_mode: false,
            amount_left: 0., amount_right: 0., amount_forward: 0., amount_backward: 0., amount_up: 0.,
            amount_down: 0., rotate_horizontal: 0., rotate_vertical: 0., scroll: 0., speed , tui}
    }

    pub fn process_keyboard_tui(&mut self, key: &KeyEvent) -> bool {
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
            KeyCode::Char('x') => {
                self.amount_up += amount;
            }
            KeyCode::Char('z') => {
                self.amount_down += amount;
            }
            KeyCode::Char('p') => {
                self.presentation_mode = !self.presentation_mode;
            }
            KeyCode::Char('r') => {
                let p = cgmath::Vector3{x:0.0, y:0.0, z:1.0};
                self.rotation = cgmath::Quaternion::from_axis_angle(p.normalize(), cgmath::Deg(0.0));
            }
            KeyCode::Modifier(_) => {}
            _ => {}
        }

        return true;

    }

    pub fn process_keyboard(&mut self, key: VirtualKeyCode, state:ElementState) -> bool {
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
            VirtualKeyCode::P => {
                self.presentation_mode = !self.presentation_mode;
                true
            }
            VirtualKeyCode::R => {
                let p = cgmath::Vector3{x:0.0, y:0.0, z:1.0};
                self.rotation = cgmath::Quaternion::from_axis_angle(p.normalize(), cgmath::Deg(0.0));
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
            _ => true,
        }
    }

    pub fn update_model(&mut self, dt: std::time::Duration) -> Vec<crate::wgpu::instance::InstanceRaw>{
        let dt = dt.as_secs_f32();

        self.position.z +=  (self.amount_forward - self.amount_backward) * self.speed * dt;
        self.position.x +=  (self.amount_right - self.amount_left) * self.speed * dt;
        self.position.y += (self.amount_up - self.amount_down) * self.speed * dt;


        if self.tui {
            self.amount_down = 0.0;
            self.amount_up = 0.0;
            self.amount_left = 0.0;
            self.amount_right =  0.0;
            self.amount_forward = 0.0;
            self.amount_backward = 0.0;
        }

        if self.presentation_mode {
            self.theta += PI * dt * 0.2;
            let rad = cgmath::Rad(self.theta);
            self.rotation = cgmath::Quaternion::from_angle_y(rad) * cgmath::Quaternion::from_angle_x(rad);
        }

        let instances = vec![Instance{position: self.position.clone(), rotation:self.rotation.clone()}];

        let data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();

        return data;
    }
}
