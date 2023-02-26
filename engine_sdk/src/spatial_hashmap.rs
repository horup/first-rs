use std::time::Instant;

use glam::{Vec2, vec2, vec3};
use slotmap::SlotMap;
use crate::{SpriteId, Entities, Sprite};
use flat_spatial::{Grid as FlatGrid, grid::GridHandle};


pub struct SpatialHashmap {
    grid:FlatGrid<SpriteId, [f32;2]>,
    handles:SlotMap<SpriteId, GridHandle>,
    max_radius:f32
}

impl SpatialHashmap {
    pub fn max_radius(&self) -> f32 {
        self.max_radius
    }
    pub fn new(sprites:&Entities<SpriteId, Sprite>) -> Self {
        let cell_size = 8;
        let grid = FlatGrid::new(cell_size);
        let mut spatial = Self {
            grid,
            handles:SlotMap::default(),
            max_radius:1.0
        };

        for (sprite_id, sprite) in sprites.iter() {
            spatial.update_pos(sprite_id, sprite.pos.truncate());
            spatial.max_radius = if spatial.max_radius < sprite.radius { sprite.radius } else { spatial.max_radius };
        }

        spatial
    }

    pub fn update_pos(&mut self, id:SpriteId, pos:Vec2) {
        if let Some(handle) = self.handles.get(id) {
            self.grid.remove_maintain(*handle);
        }

        let key = self.grid.insert([pos.x, pos.y], id);
        self.handles.insert(key);
    }

    pub fn query_around(&self, pos:Vec2, radius:f32, results:&mut Vec<SpriteId>) {
        results.clear();
        for (handle, _) in self.grid.query_around([pos.x, pos.y], radius) {
            if let Some((_, sprite_id)) = self.grid.get(handle) {
                results.push(sprite_id.clone());
            }
        }
    }
}


#[test]
fn test() {
    let mut sprites = Entities::default();
    let size = 1024;
    let now = Instant::now();
    for y in 0..size {
        for x in 0..size {
            sprites.spawn(Sprite {
                pos:vec3(x as f32, y as f32, 0.0),
                ..Default::default()
            });
        }
    }
    let elapsed = Instant::now() - now;
    println!("init sprites took {}ms with {} sprites", elapsed.as_millis(), sprites.len());
    
    let now = Instant::now();
    let mut spatial = SpatialHashmap::new(&sprites);
    let elapsed = Instant::now() - now;
    println!("init spatial took {}ms", elapsed.as_millis());

    let now = Instant::now();
    for (id, sprite) in sprites.iter_mut() {
        sprite.pos.x = size as f32 / 2.0;
        sprite.pos.y = size as f32 / 2.0;
        spatial.update_pos(id, sprite.pos.truncate());
    }
    let elapsed = Instant::now() - now;
    println!("updating all sprite positions took {}ms", elapsed.as_millis());
    
    /*let mut res = Vec::with_capacity(16);
    let now = Instant::now();
    spatial.query_around(vec2(size as f32 / 2.0, size as f32 / 2.0 as f32), 1.0, &mut res);
    let elapsed = Instant::now() - now;
    println!("query around took {}ms and found {} sprites", elapsed.as_millis(), res.len());*/
}