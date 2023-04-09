use engine_sdk::{Engine, DrawTextParams, glam::vec2, Color};

pub fn diagnostics_collect(engine:&mut dyn Engine, prev_time:&mut f64, frames:&mut f32, fps:&mut f32) {
    *frames += 1.0;
    let time = engine.time();
    if time > *prev_time + 1.0 {
        *prev_time = time;
        *fps = *frames;
        *frames = 0.0;
    }
}

pub fn diagnostics_render(engine:&mut dyn Engine, fps:&f32) {
    let screen_size = engine.screen_size();
    let dt = engine.dt() * 1000.0;
    engine.draw_text(DrawTextParams {
        screen_pos: vec2(screen_size.x, 0.0),
        text: format!("FPS {}", fps),
        scale: 16.0,
        color: Color::RED,
        horizontal_align: engine_sdk::HorizontalAlign::Right,
        vertical_align: engine_sdk::VerticalAlign::Top,
    });
}
