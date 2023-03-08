use engine_sdk::{Camera, Entities, SpriteId, Sprite, Grid, Tile, Components, World};
use serde::{Serialize, Deserialize};
use crate::{components::*, textures, systems::Flash};


#[derive(Clone, Serialize, Deserialize, Default)]
pub struct State {
    pub camera: Camera,
    pub sprites: Entities<SpriteId, Sprite>,
    pub grid: Grid<Tile>,
    pub player_id: Option<SpriteId>,
    pub items: Components<SpriteId, Item>, 
    pub doors: Components<SpriteId, Door>,
    pub effectors: Components<SpriteId, Effector>,
    pub flash:Flash,
    pub players: Components<SpriteId, Player>,
    pub activators: Components<SpriteId, Activator>,
    pub mobs: Components<SpriteId, Mob>
}

impl State {
    pub fn as_world(&mut self) -> World {
        let mut world = World::new(&self.sprites, &self.grid);
        world.ceiling_texture = textures::CEILING_GREY;
        world.floor_texture = textures::FLOOR_GREY;
        world
    }

    pub fn player_thing(&self) -> Option<PlayerThing> {
        let id = self.player_id?;
        let player = self.players.get_mut(id)?;
        let sprite = self.sprites.get_mut(id)?;
        Some(PlayerThing { id, sprite, player })
    }

    pub fn mob_thing(&self, id:SpriteId) -> Option<MobThing> {
        let sprite = self.sprites.get_mut(id)?;
        let mob = self.mobs.get_mut(id)?;
        Some(MobThing { id, sprite, mob })
    }

    pub fn item_thing(&self, id:SpriteId) -> Option<ItemThing> {
        let sprite = self.sprites.get_mut(id)?;
        let item = self.items.get_mut(id)?;
        Some(ItemThing { id, sprite, item })
    }
}

pub struct PlayerThing<'a> {
    pub id:SpriteId,
    pub sprite:&'a mut Sprite,
    pub player:&'a mut Player
}

pub struct MobThing<'a> {
    pub id:SpriteId,
    pub sprite:&'a mut Sprite,
    pub mob:&'a mut Mob
}

pub struct ItemThing<'a> {
    pub id:SpriteId,
    pub sprite:&'a mut Sprite,
    pub item:&'a mut Item
}

