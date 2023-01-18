use glam::Vec2;
use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
pub struct Rect2 {
    pub x:f32,
    pub y:f32,
    pub w:f32,
    pub h:f32
}

impl Rect2 {
    pub fn new(x:f32, y:f32, w:f32, h:f32) -> Self {
        Self {
            x,y,w,h
        }
    }

    pub fn contains(&self, p:&Vec2) -> bool {
        if p.x >= self.x && p.x <= self.x + self.w
        && p.y >= self.y && p.y <= self.y + self.h
        {
            return true;
        }
        
        false
    }
}