use engine_sdk::{
    glam::vec2,
    registry::{Facade, Registry},
    Color, DrawLineParams, DrawRectParams, DrawTextParams, Engine, HorizontalAlign, math::smootherstep, Pic,
};

use crate::{components::PlayerState, textures, PiggyFacade, PlayerEntity, items};

pub fn ui_system(registry: &mut Registry, engine: &mut dyn Engine) {
    let facade = registry.facade::<PiggyFacade>();
    // draw ui
    let _margin = vec2(16.0, 16.0);
    let center = engine.screen_size() / 2.0;

    let l = 8.0;
    let w = 1.0;
    engine.draw_line(DrawLineParams {
        begin: center + vec2(-l, 0.0),
        end: center + vec2(l, 0.0),
        line_width: w,
        color: Color::WHITE,
    });

    engine.draw_line(DrawLineParams {
        begin: center + vec2(0.0, -l),
        end: center + vec2(0.0, l),
        line_width: w,
        color: Color::WHITE,
    });

    for e in facade.query::<PlayerEntity>() {
        engine.draw_text(DrawTextParams {
            screen_pos: vec2(16.0, 16.0),
            text: format!(
                "Pokemon Cards: {:?}",
                e.player.inventory.amount(items::POKEMONCARD) as u32
            ),
            color: Color::WHITE,
            scale: 16.0,
            ..Default::default()
        });

        let size = vec2(32.0, 32.0);

        if e.player.inventory.has(items::KEY_BLUE) {
            engine.draw_rect(DrawRectParams {
                pos: vec2(16.0, 32.0),
                size,
                pic:Some(items::KEY_BLUE),
                ..Default::default()
            });
        }

        if e.player.inventory.has(items::KEY_GOLD) {
            engine.draw_rect(DrawRectParams {
                pos: vec2(16.0, 32.0 + size.y),
                size,
                pic:Some(items::KEY_GOLD),
                ..Default::default()
            });
        }

        fn draw_cought(engine: &mut dyn Engine) {
            let size = engine.screen_size();
            engine.draw_text(DrawTextParams {
                screen_pos: vec2(size.x / 2.0, size.y / 2.0),
                text: "You were cought!!!!".to_string(),
                scale: 32.0,
                color: Color::RED,
                horizontal_align: HorizontalAlign::Center,
                ..Default::default()
            });
        }

        fn draw_won(engine: &mut dyn Engine, alpha:f32) {
            let size = engine.screen_size();
            let alpha = smootherstep(0.0, 1.0, alpha);
            engine.draw_text(DrawTextParams {
                screen_pos: vec2(size.x / 2.0, size.y / 2.0),
                text: "Gratz! you escaped!".to_string(),
                scale: 32.0,
                color: Color { r: 1.0, g: 1.0, b: 1.0, a: alpha },
                horizontal_align: HorizontalAlign::Center,
                ..Default::default()
            });
        }

        fn draw_can_respawn(engine: &mut dyn Engine) {
            let size = engine.screen_size();
            engine.draw_text(DrawTextParams {
                screen_pos: vec2(size.x / 2.0, size.y / 2.0 + 32.0),
                text: "Press SPACE to Restart...".to_string(),
                scale: 32.0,
                horizontal_align: HorizontalAlign::Center,
                color: Color::WHITE,
                ..Default::default()
            });
        }

        fn draw_can_continue(engine: &mut dyn Engine) {
            let size = engine.screen_size();
            engine.draw_text(DrawTextParams {
                screen_pos: vec2(size.x / 2.0, size.y / 2.0 + 32.0),
                text: "Press SPACE to continue...".to_string(),
                scale: 32.0,
                horizontal_align: HorizontalAlign::Center,
                color: Color::WHITE,
                ..Default::default()
            });
        }

        fn draw_fade(engine: &mut dyn Engine, alpha:f32) {
            let alpha = smootherstep(0.0, 1.0, alpha);
            engine.draw_rect(DrawRectParams {
                pos: vec2(0.0, 0.0),
                size: engine.screen_size(),
                color: Color { r: 0.0, g: 0.0, b: 0.0, a: alpha },
                pic:None
            });
        }

        fn draw_final(e:&mut dyn Engine) {
            let size = e.screen_size() / 2.0;
            e.draw_text(DrawTextParams {
                screen_pos: size,
                text: "Congratulation! You won the game!".into(),
                scale: 24.0,
                color: Color::WHITE,
                horizontal_align: engine_sdk::HorizontalAlign::Center,
                vertical_align: engine_sdk::VerticalAlign::Center,
            });
        }

        match e.player.state {
            PlayerState::BeingCought { .. } => {
                draw_cought(engine);
            },
            PlayerState::Cought { fade_out_timer } => {
                draw_fade(engine, fade_out_timer.alpha_capped());
                draw_cought(engine);
            }
            PlayerState::CanRespawn => {
                draw_fade(engine, 1.0);
                draw_cought(engine);
                draw_can_respawn(engine);
            }
            PlayerState::Escaped { fade_out_timer } => {
                let alpha = fade_out_timer.alpha();
                draw_fade(engine, alpha);
                draw_won(engine, alpha);
            }
            PlayerState::Active { fade_in_timer } => {
                let alpha = 1.0 - fade_in_timer.alpha_capped();
                draw_fade(engine, alpha);
            }
            PlayerState::CanContinue {  } => {
                draw_fade(engine, 1.0);
                draw_won(engine, 1.0);
                draw_can_continue(engine);
            }
            PlayerState::ShowFinalScore {  } => {
                draw_fade(engine, 1.0);
                draw_final(engine);
            },
        }
    }
}
