use engine_sdk::Engine;
use crate::State;

pub fn mob_system(state:&mut State, engine:&mut dyn Engine) -> Option<()> {
    let player = state.player_thing()?;

    for (id, _) in state.sprites.iter() {
        if let Some(mob_thing) = state.mob_thing(id) {
            let dir = (player.sprite.pos - mob_thing.sprite.pos).normalize_or_zero();
            mob_thing.mob.dir = dir.truncate();
            mob_thing.sprite.vel = dir;
        }
    }

    None
}