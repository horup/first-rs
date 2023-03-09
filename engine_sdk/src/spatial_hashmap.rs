
use glam::{Vec2};
use slotmap::SlotMap;
use crate::{SpriteId, CopySlotMap, Sprite};
use flat_spatial::{Grid as FlatGrid, grid::GridHandle};

pub struct SpatialHashmap<'a> {
    sprites:&'a CopySlotMap<SpriteId, Sprite>,
    grid:FlatGrid<SpriteId, [f32;2]>,
    handles:SlotMap<SpriteId, GridHandle>,
    max_radius:f32,
    requires_update:bool
}

impl<'a> SpatialHashmap<'a> {
    pub fn max_radius(&self) -> f32 {
        self.max_radius
    }
    pub fn new(sprites:&'a CopySlotMap<SpriteId, Sprite>) -> Self {
        let cell_size = 8;
        let grid = FlatGrid::new(cell_size);
        let spatial = Self {
            sprites,
            grid,
            handles:SlotMap::default(),
            max_radius:1.0,
            requires_update:true
        };

        spatial
    }

    pub fn update_all(&mut self) {
        for (sprite_id, sprite) in self.sprites.iter() {
            self.update_one(sprite_id, sprite.pos.truncate());
            self.max_radius = if self.max_radius < sprite.radius { sprite.radius } else { self.max_radius };
        }

        self.requires_update = false;
    }

    pub fn invalidate(&mut self) {
        self.requires_update = true;
    }

    pub fn update_one(&mut self, id:SpriteId, pos:Vec2) {
        if let Some(handle) = self.handles.get(id) {
            self.grid.remove_maintain(*handle);
        }

        let key = self.grid.insert([pos.x, pos.y], id);
        self.handles.insert(key);
    }

    pub fn query_around(&mut self, pos:Vec2, radius:f32, results:&mut Vec<SpriteId>) {
        if self.requires_update {
            self.update_all();
        }

        results.clear();
        for (handle, _) in self.grid.query_around([pos.x, pos.y], radius) {
            if let Some((_, sprite_id)) = self.grid.get(handle) {
                results.push(*sprite_id);
            }
        }
    }
}