
use glam::{Vec2};
use slotmap::SlotMap;
use crate::{SpriteId, Sprite, ComponentsCopy, EntityId, Entities};
use flat_spatial::{Grid as FlatGrid, grid::GridHandle};

pub struct SpatialHashmap<'a> {
    entities:&'a Entities,
    sprites:&'a ComponentsCopy<Sprite>,
    grid:FlatGrid<EntityId, [f32;2]>,
    handles:SlotMap<EntityId, GridHandle>,
    max_radius:f32,
    requires_update:bool
}

impl<'a> SpatialHashmap<'a> {
    pub fn max_radius(&self) -> f32 {
        self.max_radius
    }
    pub fn new(entities:&'a Entities, sprites:&'a ComponentsCopy<Sprite>) -> Self {
        let cell_size = 8;
        let grid = FlatGrid::new(cell_size);
        let spatial = Self {
            entities,
            sprites,
            grid,
            handles:SlotMap::default(),
            max_radius:1.0,
            requires_update:true
        };

        spatial
    }

    pub fn update_all(&mut self) {
        for entity_id in self.entities.iter() {
            if let Some(sprite) = self.sprites.get(entity_id) {
                self.update_one(entity_id, sprite.pos.truncate());
                self.max_radius = if self.max_radius < sprite.radius { sprite.radius } else { self.max_radius };
            }
        }

        self.requires_update = false;
    }

    pub fn invalidate(&mut self) {
        self.requires_update = true;
    }

    pub fn update_one(&mut self, id:EntityId, pos:Vec2) {
        if let Some(handle) = self.handles.get(id) {
            self.grid.remove_maintain(*handle);
        }

        let key = self.grid.insert([pos.x, pos.y], id);
        self.handles.insert(key);
    }

    pub fn query_around(&mut self, pos:Vec2, radius:f32, results:&mut Vec<EntityId>) {
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