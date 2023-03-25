use std::cell::RefMut;
use engine_sdk::{world::{EntityId, Query}, Sprite};
use crate::components::{Player, Health, Item, Mob, Door};

pub struct DoorEntity<'a> {
    pub id:EntityId,
    pub sprite:RefMut<'a, Sprite>,
    pub door:RefMut<'a, Door>
}
impl<'a> Query<'a> for DoorEntity<'a> {
    fn query(world:&'a engine_sdk::world::World, id:EntityId) -> Option<Self> {
        let sprite = world.component_mut::<Sprite>(id)?;
        let door = world.component_mut::<Door>(id)?;
        Some(DoorEntity {
            id,
            sprite,
            door
        })
    }
}

pub struct PlayerEntity<'a> {
    pub id:EntityId,
    pub player:RefMut<'a, Player>,
    pub sprite:RefMut<'a, Sprite>,
    pub health:RefMut<'a, Health>
}

impl<'a> Query<'a> for PlayerEntity<'a> {
    fn query(world:&'a engine_sdk::world::World, id:EntityId) -> Option<Self> {
        let player = world.component_mut(id)?;
        let health = world.component_mut(id)?;
        let sprite = world.component_mut(id)?;

        Some(Self {
            id,
            player,
            sprite,
            health
        })
    }
}

pub struct ItemEntity<'a> {
    pub id:EntityId,
    pub sprite:RefMut<'a, Sprite>,
    pub item:RefMut<'a, Item>
}

impl<'a> Query<'a> for ItemEntity<'a> {
    fn query(world:&'a engine_sdk::world::World, id:EntityId) -> Option<Self> {
        let sprite = world.component_mut(id)?;
        let item = world.component_mut(id)?;
        Some(Self {
            id,
            sprite,
            item
        })
    }
}

pub struct MobEntity<'a> {
    pub id:EntityId,
    pub sprite:RefMut<'a, Sprite>,
    pub mob:RefMut<'a, Mob>
}

impl<'a> Query<'a> for MobEntity<'a>{
    fn query(world:&'a engine_sdk::world::World, id:EntityId) -> Option<Self> {
        let sprite = world.component_mut::<Sprite>(id)?;
        let mob = world.component_mut::<Mob>(id)?;
        Some(Self {
            id,
            sprite,
            mob
        })
    }
}

/*use engine_sdk::{Camera, Sprite, Grid, Tile, World, Components, Entities, EntityId, Collision};
use serde::{Serialize, Deserialize};
use crate::{components::*, textures, systems::Flash};

/* */
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct State {
    pub entities: Entities,
    pub camera: Camera,
    pub sprites: Components<Sprite>,
    pub grid: Grid<Tile>,
    pub player_id: Option<EntityId>,
    pub items: Components<Item>, 
    pub doors: Components<Door>,
    pub effectors: Components<Effector>,
    pub flash:Flash,
    pub players: Components<Player>,
    pub activators: Components<Activator>,
    pub mobs: Components<Mob>,
    pub collisions:Vec<Collision>,
    pub healths:Components<Health>
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
        let player = self.players.get_mut2(id)?;
        let sprite = self.sprites.get_mut2(id)?;
        let health = self.healths.get_mut2(id)?;
        Some(PlayerEntity { id, sprite, player, health })
    }

    pub fn mob_entity(&self, id:EntityId) -> Option<MobEntity> {
        let sprite = self.sprites.get_mut2(id)?;
        let mob = self.mobs.get_mut2(id)?;
        Some(MobEntity { id, sprite, mob })
    }

    pub fn item_entity(&self, id:EntityId) -> Option<ItemEntity> {
        let sprite = self.sprites.get_mut2(id)?;
        let item = self.items.get_mut2(id)?;
        Some(ItemEntity { id, sprite, item })
    }

    pub fn activator_entity(&self, id:EntityId) -> Option<ActivatorEntity> {
        let sprite = self.sprites.get_mut2(id)?;
        let activator = self.activators.get_mut2(id)?;
        Some(ActivatorEntity { id, sprite, activator })
    }

    pub fn door_entity(&self, id:EntityId) -> Option<DoorEntity> {
        let sprite = self.sprites.get_mut2(id)?;
        let door = self.doors.get_mut2(id)?;
        Some(DoorEntity { id, sprite, door })
    }

    pub fn activatee_entity(&self, id:EntityId) -> Option<ActivateeEntity> {
        let player_thing = self.player_entity()?;
        let sprite = self.sprites.get_mut2(id)?;
        let player = self.players.get_mut2(id);
        let mob = self.mobs.get_mut2(id);
        if player.is_some() || mob.is_some() {
            return Some(ActivateeEntity { id, sprite, player_thing});
        }
        
        None
    }
}

pub struct PlayerEntity<'a> {
    pub id:EntityId,
    pub health:&'a mut Health,
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

*/