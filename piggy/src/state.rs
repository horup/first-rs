use engine_sdk::{Camera, CopySlotMap, SpriteId, Sprite, Grid, Tile, CopySecondaryMap, World};
use serde::{Serialize, Deserialize};
use crate::{components::*, textures, systems::Flash};


#[derive(Clone, Serialize, Deserialize, Default)]
pub struct State {
    pub camera: Camera,
    pub sprites: CopySlotMap<SpriteId, Sprite>,
    pub grid: Grid<Tile>,
    pub player_id: Option<SpriteId>,
    pub items: CopySecondaryMap<SpriteId, Item>, 
    pub doors: CopySecondaryMap<SpriteId, Door>,
    pub effectors: CopySecondaryMap<SpriteId, Effector>,
    pub flash:Flash,
    pub players: CopySecondaryMap<SpriteId, Player>,
    pub activators: CopySecondaryMap<SpriteId, Activator>,
    pub mobs: CopySecondaryMap<SpriteId, Mob>
}

impl State {
    pub fn as_world(&self) -> World {
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

    pub fn activator_thing(&self, id:SpriteId) -> Option<ActivatorThing> {
        let sprite = self.sprites.get_mut(id)?;
        let activator = self.activators.get_mut(id)?;
        Some(ActivatorThing { id, sprite, activator })
    }

    pub fn door_thing(&self, id:SpriteId) -> Option<DoorThing> {
        let sprite = self.sprites.get_mut(id)?;
        let door = self.doors.get_mut(id)?;
        Some(DoorThing { id, sprite, door })
    }

    pub fn activatee_thing(&self, id:SpriteId) -> Option<ActivateeThing> {
        let player_thing = self.player_thing()?;
        let sprite = self.sprites.get_mut(id)?;
        let player = self.players.get_mut(id);
        let mob = self.mobs.get_mut(id);
        if player.is_some() || mob.is_some() {
            return Some(ActivateeThing { id, sprite, player_thing});
        }
        
        None
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

pub struct DoorThing<'a> {
    pub id:SpriteId,
    pub sprite:&'a mut Sprite,
    pub door:&'a mut Door
}

pub struct ActivatorThing<'a> {
    pub id:SpriteId,
    pub sprite:&'a mut Sprite,
    pub activator:&'a mut Activator
}

pub struct ActivateeThing<'a> {
    pub id:SpriteId,
    pub sprite:&'a mut Sprite,
    pub player_thing:PlayerThing<'a>
}

