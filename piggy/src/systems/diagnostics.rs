use engine_sdk::{Engine, DrawTextParams, glam::vec2, Color};

#[derive(Default)]
pub struct DiagnosticsSystem {
    fps:f32,
    frames:f32,
    prev_time:f64
}

impl DiagnosticsSystem {
    pub fn calculate_fps(&mut self, engine:&mut dyn Engine) {
        self.frames += 1.0;
        let time = engine.time();
        if time > self.prev_time + 1.0 {
            self.prev_time = time;
            self.fps = self.frames;
            self.frames = 0.0;
        }
    }
    pub fn render(&self, engine:&mut dyn Engine) {
        let screen_size = engine.screen_size();
        let dt = engine.dt() * 1000.0;
        engine.draw_text(DrawTextParams {
            screen_pos: vec2(screen_size.x, 0.0),
            text: format!("FPS {}", self.fps),
            scale: 16.0,
            color: Color::RED,
            horizontal_align: engine_sdk::HorizontalAlign::Right,
            vertical_align: engine_sdk::VerticalAlign::Top,
        });
    }
}