use engine_sdk::{registry::{Facade, Registry, Components}, Sprite};

pub struct PiggyFacade<'a> {
    pub registry:&'a Registry,
    pub sprites:Components<'a, Sprite>
}

impl<'a> Facade<'a> for PiggyFacade {

}