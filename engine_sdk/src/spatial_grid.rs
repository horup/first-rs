use glam::Vec2;
use slotmap::SlotMap;
use crate::{SpriteId, Entities, Sprite};
use flat_spatial::{Grid as FlatGrid, grid::GridHandle};


pub struct SpatialHashmap {
    grid:FlatGrid<SpriteId, [f32;2]>,
    handles:SlotMap<SpriteId, GridHandle>
}

impl SpatialHashmap {
    pub fn max_radius(&self) -> f32 {
        1.0
    }
    pub fn new(sprites:&Entities<SpriteId, Sprite>) -> Self {
        let cell_size = 8;
        let grid = FlatGrid::new(cell_size);
        let mut spatial = Self {
            grid,
            handles:SlotMap::default()
        };

        for (sprite_id, sprite) in sprites.iter() {
            spatial.update(sprite_id, sprite.pos.truncate());
        }

        spatial
    }

    pub fn update(&mut self, id:SpriteId, pos:Vec2) {
        let new_handle = self.grid.insert([pos.x, pos.y], id);
        dbg!("was inserted");
        if let Some(handle) = self.handles.get(id) {
            dbg!("has handle");
            if *handle != new_handle {
                self.grid.remove_maintain(*handle);
                dbg!("was removed");
            }
        }
    }
}


#[test]
fn test() {
    let mut sprites = Entities::default();
    sprites.spawn(Sprite {
        ..Default::default()
    });
    let mut spatial = SpatialHashmap::new(&sprites);
    for (id, sprite) in sprites.iter_mut() {
        sprite.pos.x = 100000000000000.0;
        spatial.update(id, sprite.pos.truncate());
    }

    dbg!("hi");
}