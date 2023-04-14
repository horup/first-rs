use crate::Editor;

impl engine_sdk::Editor for Editor {
    fn def_wall(&mut self, atlas_def:engine_sdk::AtlasDef) {
        self.walls.push(atlas_def);
    }

    fn def_entity(&mut self, atlas_def:engine_sdk::AtlasDef) {
        self.entities.push(atlas_def);
    }
}