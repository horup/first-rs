use engine_sdk::{LoadAtlasParams, Engine, image, Atlas, registry::Registry, Def, Pic};

use crate::{textures, sounds, components::{Event, StartEvent}, atlases};

pub fn init_system(r:&mut Registry, engine:&mut dyn Engine) {
    let start = engine.time();
    macro_rules! load_atlas {
        ($id:expr, $path:expr, $atlas:expr) => {
            engine.load_atlas($id, &image::load_from_memory(include_bytes!($path)).unwrap(), LoadAtlasParams {
                atlas:$atlas,
                ..Default::default()
            });
        };
    }

    macro_rules! sound {
        ($id:expr, $path:expr) => {
            engine.load_sound($id, include_bytes!($path));
        };
    }

    load_atlas!(atlases::TILES, "../../assets/atlases/tiles.png", Atlas::new(8, 8, 192));
    load_atlas!(atlases::CREATURES, "../../assets/atlases/creatures.png", Atlas::new(8, 8, 192));
    load_atlas!(atlases::DECORATIONS, "../../assets/atlases/decorations.png", Atlas::new(8, 8, 192));
    load_atlas!(atlases::ITEMS, "../../assets/atlases/items.png", Atlas::new(8, 8, 192));
    load_atlas!(atlases::MARKERS, "../../assets/atlases/markers.png", Atlas::new(8, 8, 192));
    load_atlas!(atlases::DOORS, "../../assets/atlases/doors.png", Atlas::new(8, 8, 192));
    if let Some(editor) = engine.editor() {
        for i in 0..7 {
            editor.def_wall(Def::new(atlases::TILES, i, "tile"));
        }

        // markers
        editor.def_entity(Def::new(atlases::MARKERS, 0, "spawn_player"));
        editor.def_entity(Def::new(atlases::MARKERS, 1, "exit"));
        editor.def_entity(Def::new(atlases::MARKERS, 2, "blocks"));

        // doors
        for i in 0..3 {
            editor.def_entity(Def::new(atlases::DOORS, i, "door"));
        }

        // items
        for i in 0..5 {
            editor.def_entity(Def::new(atlases::ITEMS, i, "item"));
        }

        // creatures
        editor.def_entity(Def::new(atlases::CREATURES, 2, "mob"));
        editor.def_entity(Def::new(atlases::CREATURES, 3, "mob"));
        editor.def_entity(Def::new(atlases::CREATURES, 4, "mob"));
        
        // decorations
        for i in 0..1 {
            editor.def_entity(Def::new(atlases::DECORATIONS, i, "decoration"));
        }
    }
    
    sound!(sounds::PICKUP, "../../assets/audio/pickup.ogg");
    sound!(sounds::PICKUP_KEY, "../../assets/audio/pickup_key.ogg");
    sound!(sounds::DOOR_OPEN, "../../assets/audio/door_open.ogg");
    sound!(sounds::DOOR_CLOSE, "../../assets/audio/door_close.ogg");
    //sound!(sounds::MUSIC01, "../../assets/music/iwan_gabovitch.ogg");
    sound!(sounds::COUGHT, "../../assets/audio/cought.ogg");
    sound!(sounds::WIN, "../../assets/audio/win.ogg");
    sound!(sounds::LOSE, "../../assets/audio/win.ogg");
    sound!(sounds::FINAL, "../../assets/audio/final.ogg");
    sound!(sounds::TRAP, "../../assets/audio/trap.ogg");
    
    r.spawn().attach(Event::Start(StartEvent::default()));

    let took = engine.time() - start;
    // println!("init() took {}s", took);
}