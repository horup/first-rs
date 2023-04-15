use engine_sdk::{LoadAtlasParams, Engine, image, Atlas, registry::Registry, Def, Pic};

use crate::{textures, sounds, components::{Event, StartEvent}};

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

   /* macro_rules! thing {
        ($id:expr, $path:expr, $atlas:expr) => {
            engine.load_atlas
            ($id, &image::load_from_memory(include_bytes!($path)).unwrap(), LoadAtlasParams {
                atlas:$atlas,
                editor_props:EditorProps::thing(),
                ..Default::default()
            });
        };
    }*/

    macro_rules! sound {
        ($id:expr, $path:expr) => {
            engine.load_sound($id, include_bytes!($path));
        };
    }

    load_atlas!(textures::WALLS, "../../assets/textures/walls.png", Atlas::new(8, 8));
    load_atlas!(textures::MARKERS, "../../assets/textures/markers.png", Atlas::new(8, 8));
    if let Some(editor) = engine.editor() {
        editor.def_wall(Def {
            pic:Pic::new(textures::WALLS, 0),
            tags:vec!["Walls".into()]
        });
        editor.def_wall(Def {
            pic:Pic::new(textures::WALLS, 1),
            tags:vec!["Walls".into()]
        });

        editor.def_entity(Def { pic:Pic::new(textures::MARKERS, 0), tags: vec!["SpawnPlayer".into(), "Marker".into()] });
        editor.def_entity(Def { pic:Pic::new(textures::MARKERS, 1), tags: vec!["ExitLevel".into(), "Marker".into()] });
    }
    
    
    //wall!(textures::WALL_BRICK, "../../assets/textures/wall_brick.png");
    //wall!(textures::WALL_BUSH, "../../assets/textures/wall_bush.png");
    //wall!(textures::WALL_WHITE, "../../assets/textures/wall_white.png");
    /*wall!(textures::WALLS, "../../assets/textures/wall_wood_dark.png");

    thing!(textures::THING_MARKER_SPAWN_PLAYER, "../../assets/textures/thing_player.png", Atlas::new(1, 1));
    thing!(textures::THING_VIKTOR, "../../assets/textures/thing_player_viktor.png", Atlas::new(1, 1));
    thing!(textures::THING_WILLIAM, "../../assets/textures/thing_player_william.png", Atlas::new(2, 1));
    thing!(textures::THING_DOOR_BLUE, "../../assets/textures/thing_door_blue.png", Atlas::new(1, 1));
    thing!(textures::THING_DOOR_WHITE, "../../assets/textures/thing_door_white.png", Atlas::new(1, 1));
    thing!(textures::THING_DOOR_GOLD, "../../assets/textures/thing_door_gold.png", Atlas::new(1, 1));
    thing!(textures::THING_ITEM_POKEMONCARD, "../../assets/textures/thing_item_pokemoncard.png", Atlas::new(1, 1));
    thing!(textures::THING_ITEM_KEY_GOLD, "../../assets/textures/thing_item_key_gold.png", Atlas::new(1, 1));
    thing!(textures::THING_ITEM_KEY_BLUE, "../../assets/textures/thing_item_key_blue.png", Atlas::new(1, 1));
    thing!(textures::THING_MONSTER_PIGGY, "../../assets/textures/thing_monster_piggy.png", Atlas::new(1, 1));
    thing!(textures::THING_PLANT, "../../assets/textures/thing_plant.png", Atlas::new(1, 1));
    thing!(textures::THING_MARKER_EXIT, "../../assets/textures/thing_marker_exit.png", Atlas::new(1, 1));
    wall!(textures::FLOOR_GREY, "../../assets/textures/floor_grey.png");
    wall!(textures::CEILING_GREY, "../../assets/textures/ceiling_grey.png");


    sound!(sounds::PICKUP, "../../assets/audio/pickup.ogg");
    sound!(sounds::PICKUP_KEY, "../../assets/audio/pickup_key.ogg");
    sound!(sounds::DOOR_OPEN, "../../assets/audio/door_open.ogg");
    sound!(sounds::DOOR_CLOSE, "../../assets/audio/door_close.ogg");
    //sound!(sounds::MUSIC01, "../../assets/music/iwan_gabovitch.ogg");
    sound!(sounds::COUGHT, "../../assets/audio/cought.ogg");
    sound!(sounds::WIN, "../../assets/audio/win.ogg");
    sound!(sounds::LOSE, "../../assets/audio/win.ogg");
    sound!(sounds::FINAL, "../../assets/audio/final.ogg");*/
    
    //r.spawn().attach(Event::Start(StartEvent::default()));

    let took = engine.time() - start;
    println!("init() took {}s", took);
}