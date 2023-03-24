use glam::{Vec2};
use slotmap::SlotMap;
use flat_spatial::{Grid as FlatGrid, grid::GridHandle};
use world::{EntityId, World};
use crate::Sprite;

pub struct SpatialHashmap<'a> {
    world:&'a World,
    handles:SlotMap<EntityId, GridHandle>,
    max_radius:f32,
    requires_update:bool,
    grid:FlatGrid<EntityId, [f32;2]>,
}

impl<'a> SpatialHashmap<'a> {
    pub fn max_radius(&self) -> f32 {
        self.max_radius
    }
    pub fn new(world:&'a World) -> Self {
        let cell_size = 8;
        let grid = FlatGrid::new(cell_size);

        Self {
            grid,
            world,
            handles:SlotMap::default(),
            max_radius:1.0,
            requires_update:true
        }
    }

    pub fn update_all(&mut self) {
        for e in self.world.entities() {
            if let Some(sprite) = e.get::<Sprite>() {
                self.update_one(e.id(), sprite.pos.truncate());
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