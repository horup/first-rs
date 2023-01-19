use std::collections::HashMap;

use engine_sdk::glam::Vec2;

#[derive(Default)]
pub struct Input {
    pub mouse_pos:Vec2,
    pub mouse_pressed:[bool;4],
    pub keys_pressed:HashMap<u32, bool>,
    pub keys_just_pressed:Vec<u32>,
    pub mouse_wheel_delta:Vec2
}

impl Input {
    pub fn clear(&mut self) {
        self.keys_just_pressed.clear();
        self.mouse_wheel_delta = Vec2::default();
    }
}
