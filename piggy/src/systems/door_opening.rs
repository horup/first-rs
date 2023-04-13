use std::f32::consts::PI;
use engine_sdk::{Engine, glam::{vec3}, registry::{Registry, Facade}, Tilemap, SoundEmitter};
use crate::{DoorEntity, PiggyFacade, sounds};

pub fn door_system(registry:&mut Registry, engine:&mut dyn Engine) {
    {
        let tilemap = &mut registry.singleton_mut::<Tilemap>().unwrap().grid;
        // update doors 
        let dt = engine.dt();
        for mut e in registry.facade::<PiggyFacade>().query::<DoorEntity>() {
            let speed = 2.0;
            e.door.openess += speed * e.door.direction * dt;
            if e.door.openess < 0.0 {
                e.door.openess = 0.0;
                e.door.direction = 0.0;
                registry.push(|r|{
                    r.spawn().attach(SoundEmitter::once(sounds::DOOR_CLOSE));
                });
            } else if e.door.openess > 1.0 {
                e.door.openess = 1.0;
                e.door.direction = 0.0;
                e.door.close_timer = 0.0;
            }
            
            if e.door.is_open() {
                e.door.close_timer += dt;
                e.sprite.clips = true;
                if e.door.close_timer > e.door.time_to_start_closing() {
                    e.door.close();
                }
            } 

            let dir = e.sprite.facing - PI / 2.0;
            let v = vec3(dir.cos(), dir.sin(), 0.0);
            e.sprite.pos = e.door.pos + v * e.door.openess;     

            if let Some(tile) = tilemap.get_mut(e.door.pos.as_ivec3().truncate().into()) {
                if e.door.openess > 0.5 {
                    tile.clips = false;
                } else {
                    tile.clips = true;
                }   
            }
        }
    }

    registry.execute();
}