use engine_sdk::{Engine, DrawRectParams, Color, glam::vec2, world::{World, Singleton}};
use serde::{Serialize, Deserialize};

use crate::Global;


#[derive(Default, Serialize, Deserialize, Clone, Copy)]
pub struct Flash {
    pub flash_timer_sec:f32,
    pub flash_timer_start:f32,
    pub flash_max:f32,
}

impl Flash {
    pub fn flash(&mut self, duration_sec:f32, flash_max:f32) {
        self.flash_timer_sec = duration_sec;
        self.flash_timer_start = duration_sec;
        self.flash_max = flash_max;
    }

    pub fn alpha(&self) -> f32 {
        if self.flash_timer_start > 0.0 && self.flash_timer_sec > 0.0 {
            let a = self.flash_timer_sec / self.flash_timer_start;
            let a = a * self.flash_max;
            return a;
        }

        0.0
    }
}

pub fn render_flash_system(world:&mut World, engine:&mut dyn Engine) {
    let global = world.singleton_mut::<Global>().unwrap();
    let screen = engine.screen_size();
    let alpha = global.flash.alpha();
    if alpha > 0.0 {
        engine.draw_rect(DrawRectParams {
            pos: vec2(0.0, 0.0),
            size: screen,
            color: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: alpha,
            },
            ..Default::default()
        });
    }

    global.flash.flash_timer_sec -= engine.dt();
    global.flash.flash_timer_sec = global.flash.flash_timer_sec.max(0.0);
}