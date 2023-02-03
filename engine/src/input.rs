use std::collections::HashMap;

use engine_sdk::glam::Vec2;
use winit::event::VirtualKeyCode;

#[derive(Default)]
pub struct Input {
    pub mouse_pos:Vec2,
    pub mouse_pressed:[bool;4],
    pub keys_pressed:HashMap<VirtualKeyCode, bool>,
    pub keys_just_pressed:Vec<VirtualKeyCode>,
    pub mouse_wheel_delta:Vec2,
    pub mouse_motion:Vec2
}

impl Input {
    pub fn clear(&mut self) {
        self.keys_just_pressed.clear();
        self.mouse_wheel_delta = Vec2::default();
        self.mouse_motion = Vec2::default();
    }
}
