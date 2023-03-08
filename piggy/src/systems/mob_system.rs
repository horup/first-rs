use engine_sdk::Engine;
use crate::State;

pub fn mob_system(state:&mut State, engine:&mut dyn Engine) -> Option<()> {
    let dt = engine.dt();
    let player_id = state.player_id?;
    let player_sprite = state.sprites.get(player_id)?;

    for (id, mob_sprite) in state.sprites.iter_mut() {
        if let Some(mob) = state.mobs.get_mut(id) {
            let dir = (player_sprite.pos - mob_sprite.pos).normalize_or_zero();
            mob.dir = dir.truncate();
            mob_sprite.vel = dir;
        }
    }

    None
}