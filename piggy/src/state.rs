use engine_sdk::{Camera, Sprite, Grid, Tile, World, ComponentsCopy, Entities, EntityId, Collision};
use serde::{Serialize, Deserialize};
use crate::{components::*, textures, systems::Flash};


#[derive(Clone, Serialize, Deserialize, Default)]
pub struct State {
    pub entities: Entities,
    pub camera: Camera,
    pub sprites: ComponentsCopy<Sprite>,
    pub grid: Grid<Tile>,
    pub player_id: Option<EntityId>,
    pub items: ComponentsCopy<Item>, 
    pub doors: ComponentsCopy<Door>,
    pub effectors: ComponentsCopy<Effector>,
    pub flash:Flash,
    pub players: ComponentsCopy<Player>,
    pub activators: ComponentsCopy<Activator>,
    pub mobs: ComponentsCopy<Mob>,
    pub collisions:Vec<Collision>
}

impl State {
    pub fn as_world(&self) -> World {
        let mut world = World::new(&self.entities, &self.sprites, &self.grid);
        world.ceiling_texture = textures::CEILING_GREY;
        world.floor_texture = textures::FLOOR_GREY;
        world
    }

    pub fn player_entity(&self) -> Option<PlayerEntity> {
        let id = self.player_id?;
        let player = self.players.get_mut(id)?;
        let sprite = self.sprites.get_mut(id)?;
        Some(PlayerEntity { id, sprite, player })
    }

    pub fn mob_entity(&self, id:EntityId) -> Option<MobEntity> {
        let sprite = self.sprites.get_mut(id)?;
        let mob = self.mobs.get_mut(id)?;
        Some(MobEntity { id, sprite, mob })
    }

    pub fn item_entity(&self, id:EntityId) -> Option<ItemEntity> {
        let sprite = self.sprites.get_mut(id)?;
        let item = self.items.get_mut(id)?;
        Some(ItemEntity { id, sprite, item })
    }

    pub fn activator_entity(&self, id:EntityId) -> Option<ActivatorEntity> {
        let sprite = self.sprites.get_mut(id)?;
        let activator = self.activators.get_mut(id)?;
        Some(ActivatorEntity { id, sprite, activator })
    }

    pub fn door_entity(&self, id:EntityId) -> Option<DoorEntity> {
        let sprite = self.sprites.get_mut(id)?;
        let door = self.doors.get_mut(id)?;
        Some(DoorEntity { id, sprite, door })
    }

    pub fn activatee_entity(&self, id:EntityId) -> Option<ActivateeEntity> {
        let player_thing = self.player_entity()?;
        let sprite = self.sprites.get_mut(id)?;
        let player = self.players.get_mut(id);
        let mob = self.mobs.get_mut(id);
        if player.is_some() || mob.is_some() {
            return Some(ActivateeEntity { id, sprite, player_thing});
        }
        
        None
    }
}

pub struct PlayerEntity<'a> {
    pub id:EntityId,
    pub sprite:&'a mut Sprite,
    pub player:&'a mut Player
}

pub struct MobEntity<'a> {
    pub id:EntityId,
    pub sprite:&'a mut Sprite,
    pub mob:&'a mut Mob
}

pub struct ItemEntity<'a> {
    pub id:EntityId,
    pub sprite:&'a mut Sprite,
    pub item:&'a mut Item
}

pub struct DoorEntity<'a> {
    pub id:EntityId,
    pub sprite:&'a mut Sprite,
    pub door:&'a mut Door
}

pub struct ActivatorEntity<'a> {
    pub id:EntityId,
    pub sprite:&'a mut Sprite,
    pub activator:&'a mut Activator
}

pub struct ActivateeEntity<'a> {
    pub id:EntityId,
    pub sprite:&'a mut Sprite,
    pub player_thing:PlayerEntity<'a>
}

