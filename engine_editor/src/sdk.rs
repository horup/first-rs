use crate::Editor;

impl engine_sdk::Editor for Editor {
    fn load_walls(&mut self, param:engine_sdk::LoadWallsParam) {
        self.walls.push(param.atlas);
    }

    fn load_entity(&mut self, param:engine_sdk::LoadEntityParam) {
        self.entities.push(param.atlas);
    }
}