use engine_sdk::{Camera, Entities, SpriteId, Sprite, Grid, Cell, Components, World};
use serde::{Serialize, Deserialize};
use crate::{components::*, textures, systems::Flash};


#[derive(Clone, Serialize, Deserialize, Default)]
pub struct State {
    pub camera: Camera,
    pub sprites: Entities<SpriteId, Sprite>,
    pub grid: Grid<Cell>,
    pub player_id: Option<SpriteId>,
    pub items: Components<SpriteId, Item>, 
    pub doors: Components<SpriteId, Door>,
    pub flash:Flash
}

impl State {
    pub fn as_world(&mut self) -> World {
        let mut world = World::new(&mut self.sprites, &mut self.grid);
        world.ceiling_texture = textures::CEILING_GREY;
        world.floor_texture = textures::FLOOR_GREY;
        world
    }
}