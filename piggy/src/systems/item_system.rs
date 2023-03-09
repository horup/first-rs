use engine_sdk::Engine;
use crate::{State};

pub fn item_system(state:&mut State, _engine:&mut dyn Engine) {
    if let Some(player_thing) = state.player_entity() {
        let player_pos = player_thing.sprite.pos;
        let pickup_radius = 0.5;
        let mut world = state.as_world();
        let mut near = Vec::new();
        world.query_around(player_pos.truncate(), pickup_radius, &mut near);
        for id in near.drain(..) {
            if let Some(item_thing) = state.item_entity(id) {
                let texture = item_thing.sprite.texture;
                state.entities.despawn(id);
                state.flash.flash(0.2, 0.5);
                if let Some(player_thing) = state.player_entity() {
                    player_thing.player.inventory.add(texture, 1.0);
                }
            }
        }
    }
}