use engine_sdk::{Engine, glam::{IVec2, Vec3}, glam::Vec2};
use crate::State;

pub fn mob_system(state:&mut State, _engine:&mut dyn Engine) -> Option<()> {
    let player = state.player_entity()?;

    for id in state.entities.iter() {
        if let Some(mob_entity) = state.mob_entity(id) {
            let v = player.sprite.pos - mob_entity.sprite.pos;
            let dir = v.normalize_or_zero();
            let mob_speed = 2.0;

            // check visibility to player
            let mut player_visible = true;
            state.grid.cast_ray(mob_entity.sprite.pos.truncate(), player.sprite.pos.truncate(), |visit|{
                if visit.cell.clips {
                    player_visible = false;
                }

                return visit.cell.clips;
            });

            mob_entity.mob.can_see_player = player_visible;
            
            if mob_entity.mob.can_see_player {
                mob_entity.mob.last_known_pos = Some(player.sprite.pos);
                mob_entity.sprite.vel = dir * mob_speed;
            }
            
            if let Some(last_known_pos) = mob_entity.mob.last_known_pos {
                let start = mob_entity.sprite.pos.truncate().as_ivec2();
                let end = last_known_pos.truncate().as_ivec2();
                if let Some(path) = state.grid.astar(start.into(), end.into(), |_, tile| {
                    tile.clips
                }) {
                    let path = path.iter().skip(1);
                    for (x,y) in path {
                        let p:IVec2 = IVec2::new(*x, *y);
                        let p = p.as_vec2();
                        let p = p + Vec2::new(0.5, 0.5);
                        let v = p - mob_entity.sprite.pos.truncate();
                        mob_entity.sprite.vel = v.normalize_or_zero().extend(0.0) * mob_speed;
                        break;
                    }
                }
            } else {
                // todo implement patrol
                mob_entity.sprite.vel = Vec3::default();
            }
        }
    }

    None
}