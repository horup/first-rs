use engine_sdk::{
    glam::vec2,
    registry::{Facade, Registry},
    Color, DrawLineParams, DrawRectParams, DrawTextParams, Engine, HorizontalAlign,
};

use crate::{components::PlayerState, textures, PiggyFacade, PlayerEntity};

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
                e.player.inventory.amount(textures::THING_ITEM_POKEMONCARD) as u32
            ),
            color: Color::WHITE,
            scale: 16.0,
            ..Default::default()
        });

        let size = vec2(32.0, 32.0);

        if e.player.inventory.amount(textures::THING_ITEM_KEY_BLUE) > 0.0 {
            engine.draw_rect(DrawRectParams {
                pos: vec2(16.0, 32.0),
                size,
                texture: Some(textures::THING_ITEM_KEY_BLUE),
                ..Default::default()
            });
        }

        if e.player.inventory.amount(textures::THING_ITEM_KEY_GOLD) > 0.0 {
            engine.draw_rect(DrawRectParams {
                pos: vec2(16.0, 32.0 + size.y),
                size,
                texture: Some(textures::THING_ITEM_KEY_GOLD),
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

        fn draw_can_respawn(engine: &mut dyn Engine) {
            let size = engine.screen_size();
            engine.draw_text(DrawTextParams {
                screen_pos: vec2(size.x / 2.0, size.y / 2.0 + 32.0),
                text: "Click to respawn...".to_string(),
                scale: 32.0,
                horizontal_align: HorizontalAlign::Center,
                color: Color::WHITE,
                ..Default::default()
            });
        }

        fn draw_fade_out(engine: &mut dyn Engine, alpha:f32) {
            engine.draw_rect(DrawRectParams {
                pos: vec2(0.0, 0.0),
                size: engine.screen_size(),
                color: Color { r: 0.0, g: 0.0, b: 0.0, a: alpha },
                texture: None,
                atlas_index: 0.0,
            });
        }

        draw_fade_out(engine, 0.9);

        //draw_cought(engine);
        //draw_can_respawn(engine);

        match e.player.state {
            PlayerState::Cought { timer_sec } => {
                if timer_sec < 1.0 {
                    draw_cought(engine);
                }
            }
            PlayerState::CanRespawn => {
                draw_cought(engine);
                draw_can_respawn(engine);
            }
            PlayerState::Won { fade_out_start, fade_out_timer } => {
                
            }
            _ => {}
        }
    }
}
