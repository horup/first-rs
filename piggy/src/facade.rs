use std::cell::RefMut;
use engine_sdk::{registry::{Facade, Registry, Components, EntityFacade, EntityId}, Sprite};
use crate::{components::{Door, Mob, Health, Item, Player, Effector, Activator}};

pub struct PiggyFacade<'a> {
    pub registry:&'a Registry,
    pub sprites:Components<'a, Sprite>,
    pub doors:Components<'a, Door>,
    pub mobs:Components<'a, Mob>,
    pub healths:Components<'a, Health>,
    pub items:Components<'a, Item>,
    pub players:Components<'a, Player>,
    pub effectors:Components<'a, Effector>,
    pub activators:Components<'a, Activator>
}

impl<'a> Facade<'a> for PiggyFacade<'a> {
    fn new(registry:&'a Registry) -> Self {
        Self {
            registry,
            sprites:registry.components(),
            doors: registry.components(),
            mobs: registry.components(),
            healths: registry.components(),
            items: registry.components(),
            players: registry.components(),
            effectors: registry.components(),
            activators: registry.components(),
        }
    }

    fn registry(&self) -> &'a Registry {
        self.registry
    }
}


pub struct DoorEntity<'a> {
    pub id:EntityId,
    pub sprite:RefMut<'a, Sprite>,
    pub door:RefMut<'a, Door>
}
impl<'a> EntityFacade<'a> for DoorEntity<'a> {
    type Facade = PiggyFacade<'a>;
    fn query(facade:&'a PiggyFacade<'a>, id:EntityId) -> Option<Self> {
        let sprite = facade.sprites.get_mut(id)?;
        let door = facade.doors.get_mut(id)?;
        Some(Self {
            id,
            sprite,
            door
        })
    }
}

pub struct PlayerEntity<'a> {
    pub id:EntityId,
    pub sprite:RefMut<'a, Sprite>,
    pub player:RefMut<'a, Player>,
    pub health:RefMut<'a, Health>
}

impl<'a> EntityFacade<'a> for PlayerEntity<'a> {
    type Facade = PiggyFacade<'a>;

    fn query(facade:&'a Self::Facade, id:EntityId) -> Option<Self> {
        let sprite = facade.sprites.get_mut(id)?;
        let player = facade.players.get_mut(id)?;
        let health = facade.healths.get_mut(id)?;
        Some(Self {
            id,
            sprite,
            player,
            health,
        })
    }
}

pub struct ItemEntity<'a> {
    pub id:EntityId,
    pub sprite:RefMut<'a, Sprite>,
    pub item:RefMut<'a, Item>
}

impl<'a> EntityFacade<'a> for ItemEntity<'a> {
    type Facade = PiggyFacade<'a>;


    fn query(facade:&'a Self::Facade, id:EntityId) -> Option<Self> {
        let sprite = facade.sprites.get_mut(id)?;
        let item = facade.items.get_mut(id)?;
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

impl<'a> EntityFacade<'a> for MobEntity<'a> {
    type Facade = PiggyFacade<'a>;

    fn query(facade:&'a Self::Facade, id:EntityId) -> Option<Self> {
        let sprite = facade.sprites.get_mut(id)?;
        let mob = facade.mobs.get_mut(id)?;
        Some(Self {
            id, 
            sprite,
            mob
        })
    }
}

pub struct ActivateeEntity<'a> {
    pub id:EntityId,
    pub sprite:RefMut<'a, Sprite>
}

impl<'a>  EntityFacade<'a> for ActivateeEntity<'a> {
    type Facade = PiggyFacade<'a>;

    fn query(facade:&'a Self::Facade, id:EntityId) -> Option<Self> {
        let sprite = facade.sprites.get_mut(id)?;
        if facade.players.get(id).is_some() || facade.mobs.get(id).is_some() {
            return Some(ActivateeEntity {
                id,
                sprite,
            });
        }

        None
    }

}

pub struct ActivatorEntity<'a> {
    pub id:EntityId,
    pub activator:RefMut<'a, Activator>,
    pub sprite:RefMut<'a, Sprite>,
    pub door:Option<RefMut<'a, Door>>
}

impl<'a> EntityFacade<'a> for ActivatorEntity<'a> {
    type Facade = PiggyFacade<'a>;

    fn query(facade:&'a Self::Facade, id:EntityId) -> Option<Self> {
        let sprite = facade.sprites.get_mut(id)?;
        let activator = facade.activators.get_mut(id)?;
        let door = facade.doors.get_mut(id);
        Some(Self {
            id,
            activator,
            sprite,
            door
        })
    }
}