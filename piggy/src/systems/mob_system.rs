use engine_sdk::{Engine, glam::IVec2, glam::Vec2};
use crate::State;

pub fn mob_system(state:&mut State, _engine:&mut dyn Engine) -> Option<()> {
    let player = state.player_entity()?;

    for id in state.entities.iter() {
        if let Some(mob_entity) = state.mob_entity(id) {
            let dir = (player.sprite.pos - mob_entity.sprite.pos).normalize_or_zero();
            mob_entity.mob.dir = dir.truncate();
            mob_entity.sprite.vel = dir;

            let start = mob_entity.sprite.pos.as_ivec3().truncate();
            let end = player.sprite.pos.as_ivec3().truncate();
            if let Some(path) = state.grid.astar(start.into(), end.into(), |index, tile| {
                tile.clips
            }) {
                if let Some((x, y)) = path.first() {
                    let p:IVec2 = IVec2::new(*x, *y);
                    let p = p.as_vec2();
                    let p = p + Vec2::new(0.5, 0.5);
                    let dir = p - mob_entity.sprite.pos.truncate();
                    let dir = dir.normalize_or_zero().extend(0.0);
                    mob_entity.mob.dir = dir.truncate();
                    mob_entity.sprite.vel = dir;
                }
            }
        }
    }

    None
}