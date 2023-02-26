
use glam::{Vec2};
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
                results.push(*sprite_id);
            }
        }
    }
}