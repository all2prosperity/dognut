use crate::department::control::Controller;
use crate::wgpu::wgpu_helper::State;
use ::tui::layout::Direction;
use gilrs::{Button, GamepadId, Gilrs};
use log::debug;
use pixels::Pixels;
use winit::event::VirtualKeyCode;
use winit::window::WindowId;
use winit_input_helper::WinitInputHelper;

pub mod common;
mod control;
pub mod model;
pub mod net;
pub mod pipeline;
pub mod preview;
pub mod tui;
pub mod types;
pub mod video;
pub mod view;

pub struct Game {
    pixels: Pixels,
    state: State,
    controls: Controller,
    input: WinitInputHelper,
    gilrs: Gilrs,
    gamepad: Option<GamepadId>,
    paused: bool,
    id: WindowId,
}

impl Game {
    fn new(pixels: Pixels, state: State, id: WindowId, debug: bool) -> Self {
        Self {
            pixels,
            state,
            controls: Controller::new(),
            input: WinitInputHelper::new(),
            gilrs: Gilrs::new().unwrap(), // XXX: Don't unwrap.
            gamepad: None,
            paused: false,
            id,
        }
    }

    fn update_controls(&mut self) {
        todo!();
        // // Pump the gilrs event loop and find an active gamepad
        // while let Some(gilrs::Event { id, event, .. }) = self.gilrs.next_event() {
        //     let pad = self.gilrs.gamepad(id);
        //     if self.gamepad.is_none() {
        //         debug!("Gamepad with id {} is connected: {}", id, pad.name());
        //         self.gamepad = Some(id);
        //     } else if event == gilrs::ev::EventType::Disconnected {
        //         debug!("Gamepad with id {} is disconnected: {}", id, pad.name());
        //         self.gamepad = None;
        //     }
        // }
        //
        // self.controls = {
        //     // Keyboard controls
        //     let mut left = self.input.key_held(VirtualKeyCode::Left);
        //     let mut right = self.input.key_held(VirtualKeyCode::Right);
        //     let mut fire = self.input.key_pressed(VirtualKeyCode::Space);
        //     let mut pause = self.input.key_pressed(VirtualKeyCode::Pause)
        //         | self.input.key_pressed(VirtualKeyCode::P);
        //
        //     // GamePad controls
        //     if let Some(id) = self.gamepad {
        //         let gamepad = self.gilrs.gamepad(id);
        //
        //         left |= gamepad.is_pressed(Button::DPadLeft);
        //         right |= gamepad.is_pressed(Button::DPadRight);
        //         fire |= gamepad.button_data(Button::South).map_or(false, |button| {
        //             button.is_pressed() && button.counter() == self.gilrs.counter()
        //         });
        //         pause |= gamepad.button_data(Button::Start).map_or(false, |button| {
        //             button.is_pressed() && button.counter() == self.gilrs.counter()
        //         });
        //     }
        //     self.gilrs.inc();
        //
        //     if pause {
        //         self.paused = !self.paused;
        //     }
        //
        //     let direction = if left {
        //         Direction::Left
        //     } else if right {
        //         Direction::Right
        //     } else {
        //         Direction::Still
        //     };
        //
        //     Controls { direction, fire }
        //};
    }
}
