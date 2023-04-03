use engine_sdk::{Engine, glam::{IVec2, Vec3}, glam::Vec2, registry::{Registry, Facade}, Tilemap};
use crate::{PlayerEntity, MobEntity, singletons::GameState, PiggyFacade, components::PlayerState};

pub fn mob_system(registry:&mut Registry, _engine:&mut dyn Engine) -> Option<()> {
    let facade = registry.facade::<PiggyFacade>();
    let global = registry.singleton::<GameState>().unwrap();
    let tilemap = &registry.singleton::<Tilemap>().unwrap().grid;
    let mut player_entity = facade.query::<PlayerEntity>().next()?;

    for mut mob_entity in facade.query::<MobEntity>() {
        let v = player_entity.sprite.pos - mob_entity.sprite.pos;
        let dir = v.normalize_or_zero();
        let mob_speed = 2.0;

        
        // check visibility to player
        let mut player_visible = true;
        tilemap.cast_ray(mob_entity.sprite.pos.truncate(), player_entity.sprite.pos.truncate(), |visit|{
            if visit.cell.clips {
                player_visible = false;
            }

            visit.cell.clips
        });

        mob_entity.mob.can_see_player = player_visible;
        
        if mob_entity.mob.can_see_player {
            mob_entity.mob.last_known_pos = Some(player_entity.sprite.pos);
            mob_entity.sprite.vel = dir * mob_speed;
        }
        
        if let Some(last_known_pos) = mob_entity.mob.last_known_pos {
            let start = mob_entity.sprite.pos.truncate().as_ivec2();
            let end = last_known_pos.truncate().as_ivec2();
            if let Some(path) = tilemap.astar(start.into(), end.into(), |_, tile| {
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

        // check if touching player
        global.collisions.iter().filter(|collision|collision.entity == mob_entity.id).for_each(|collision|{
            if let Some(other_entity) = collision.other_entity {
                if player_entity.id == other_entity && mob_entity.mob.is_killer {
                    player_entity.health.kill(Some(mob_entity.id));
                    player_entity.player.state.set_being_cought();
                }
            }
        });

        if player_entity.player.state.is_being_cought_or_cought() {
            mob_entity.sprite.vel = Vec3::default();
        }
    }

    None
}