use engine_sdk::{Engine, egui};

use crate::Editor;

impl Editor {
    pub fn native_update_ui(&mut self, engine:&mut dyn Engine) {
        let ctx = engine.egui().clone();
        egui::TopBottomPanel::top("top_pane").show(&ctx, |ui|{
            ui.menu_button("File", |ui|{
                if ui.button("New").clicked() {
                }
                if ui.button("Save").clicked() {

                }
                if ui.button("Load").clicked() {
                    
                } 
            });
        });
    }
}