use engine_sdk::{Engine, egui, Map};
use native_dialog::{FileDialog, MessageDialog};

use crate::Editor;

fn file_dialog<'a>() -> FileDialog<'a> {
    FileDialog::new()
    .set_location("~/Desktop")
    .add_filter("Map", &["map"])
}

fn ask<'a>(msg:&str) -> bool {
    MessageDialog::new().set_text(msg).show_confirm().unwrap()
}

impl Editor {
    pub fn native_update_ui(&mut self, engine:&mut dyn Engine) {
        let ctx = engine.egui().clone();
        egui::TopBottomPanel::top("top_pane").show(&ctx, |ui|{
            ui.menu_button("File", |ui|{
                if ui.button("New").clicked() && ask("Want to create a new map?") {
                    self.map = Map::default();
                }
                if ui.button("Save").clicked() {
                    let path = file_dialog().show_save_single_file().unwrap();
                    if let Some(path) = path {
                        if ask("Want to save?") {
                            let json = serde_json::to_string(&self.map).unwrap();
                            std::fs::write(path, json).unwrap();
                        }
                    }
                }
                if ui.button("Load").clicked() {
                    let path = file_dialog().show_open_single_file().unwrap();
                    if let Some(path) = path  {
                        if ask("Want to load?") {
                            let json = std::fs::read_to_string(path).unwrap();
                            self.map = serde_json::from_str(&json).unwrap();
                        }
                    }
                } 
            });
        });
    }
}