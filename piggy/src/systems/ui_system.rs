use engine_sdk::{Engine, DrawLineParams, glam::vec2, Color, DrawTextParams, DrawRectParams};

use crate::{State, textures};

pub fn ui_system(state:&mut State, engine:&mut dyn Engine) {
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


    if let Some(player) = state.player_id {
        if let Some(player) = state.players.get(player) {
            engine.draw_text(DrawTextParams {
                screen_pos:vec2(16.0, 16.0),
                text: format!("Pokemon Cards: {:?}", player.inventory.amount(textures::THING_ITEM_POKEMONCARD) as u32),
                color:Color::WHITE,
                scale:16.0
            });

            let size = vec2(32.0, 32.0);

            if player.inventory.amount(textures::THING_ITEM_KEY_BLUE) > 0.0 {
                engine.draw_rect(DrawRectParams { 
                    pos: vec2(16.0, 32.0), 
                    size, 
                    texture: Some(textures::THING_ITEM_KEY_BLUE), 
                    ..Default::default()
                });
            }

            if player.inventory.amount(textures::THING_ITEM_KEY_GOLD) > 0.0 {
                engine.draw_rect(DrawRectParams { 
                    pos: vec2(16.0, 32.0 + size.y), 
                    size, 
                    texture: Some(textures::THING_ITEM_KEY_GOLD), 
                    ..Default::default()
                });
            }
            
        }
    }
}