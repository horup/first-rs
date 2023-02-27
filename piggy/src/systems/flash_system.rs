use engine_sdk::{Engine, DrawRectParams, Color, glam::vec2};
use serde::{Serialize, Deserialize};
use crate::State;


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

        return 0.0;
    }
}

pub fn flash_system(state:&mut State, engine:&mut dyn Engine) {
    let screen = engine.screen_size();
    let alpha = state.flash.alpha();
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

    state.flash.flash_timer_sec -= engine.dt();
    state.flash.flash_timer_sec = state.flash.flash_timer_sec.max(0.0);
}