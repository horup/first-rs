use engine_sdk::{Engine, DrawLineParams, glam::vec2, Color, DrawTextParams, DrawRectParams, registry::{Registry, Facade}};

use crate::{textures, components::PlayerState, PlayerEntity, PiggyFacade};

pub fn ui_system(registry:&mut Registry, engine:&mut dyn Engine) {
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
            screen_pos:vec2(16.0, 16.0),
            text: format!("Pokemon Cards: {:?}", e.player.inventory.amount(textures::THING_ITEM_POKEMONCARD) as u32),
            color:Color::WHITE,
            scale:16.0
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



        fn draw_cought(engine:&mut dyn Engine) {
            let size = engine.screen_size();
            engine.draw_text(DrawTextParams {
                screen_pos: vec2(size.x / 2.0, size.y / 2.0),
                text: "You were cought!!!!".to_string(),
                scale: 32.0,
                color: Color::RED,
            });
        }

        fn draw_can_respawn(engine:&mut dyn Engine) {
            let size = engine.screen_size();
            engine.draw_text(DrawTextParams {
                screen_pos: vec2(size.x / 2.0, size.y / 2.0 + 32.0),
                text: "Click to respawn...".to_string(),
                scale: 32.0,
                color: Color::WHITE,
            });
        }

        //draw_cought(engine);
        //draw_can_respawn(engine);

        match e.player.state {
            PlayerState::Cought { timer_sec } => {
                if timer_sec < 1.0 {
                    draw_cought(engine);
                }
            },
            PlayerState::CanRespawn => {
                draw_cought(engine);
                draw_can_respawn(engine);
            },
            _ => {

            }
        }
    }
}