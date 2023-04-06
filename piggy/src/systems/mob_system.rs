use engine_sdk::{Engine, glam::{IVec2, Vec3}, glam::Vec2, registry::{Registry, Facade}, Tilemap, Grid};
use crate::{PlayerEntity, MobEntity, singletons::GameState, PiggyFacade, DoorEntity};

pub fn mob_system(registry:&mut Registry, _engine:&mut dyn Engine) -> Option<()> {
    let facade = registry.facade::<PiggyFacade>();
    let global = registry.singleton::<GameState>().unwrap();
    let tilemap = &registry.singleton::<Tilemap>().unwrap().grid;
    let mut player_entity = facade.query::<PlayerEntity>().next()?;

    let mut block_grid:Grid<bool> = Grid::new(tilemap.size());
    for y in 0..tilemap.size() as i32 {
        for x in 0..tilemap.size() as i32 {
            let blocks = block_grid.get_mut((x,y)).unwrap();
            *blocks = tilemap.get((x, y)).unwrap().clips;
        }
    }

    for door_entity in facade.query::<DoorEntity>() {
        let i = door_entity.door.pos.as_ivec3().truncate();
        if let Some(blocks) = block_grid.get_mut((i.x, i.y)) {
            *blocks = false;
        }
    }

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
            mob_entity.mob.active = true;
        }
        
        if mob_entity.mob.active {
            let last_known_pos = player_entity.sprite.pos;
            let start = mob_entity.sprite.pos.truncate().as_ivec2();
            let end = last_known_pos.truncate().as_ivec2();

            if let Some(path) = block_grid.astar(start.into(), end.into(), |_, blocks| {
                *blocks
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