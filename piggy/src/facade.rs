use std::cell::RefMut;

use engine_sdk::{registry::{Facade, Registry, Components, EntityFacade, EntityId}, Sprite};

use crate::components::{Door, Mob, Health, Item, Player, Effector, Activator};

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
    fn query(registry:&'a engine_sdk::registry::Registry, id:EntityId) -> Option<Self> {
        let sprite = registry.component_mut::<Sprite>(id)?;
        let door = registry.component_mut::<Door>(id)?;
        Some(DoorEntity {
            id,
            sprite,
            door
        })
    }
}