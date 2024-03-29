use std::{rc::Rc, io::Cursor};

use crate::{Engine, Load};
use engine_sdk::{
    self,
    glam::{Vec2},
    image::DynamicImage,
    DrawRectParams, TextureAtlas, Event, LoadAtlasParams, registry::Registry, SoundEmitter,
};
use kira::{sound::static_sound::{StaticSoundData, StaticSoundSettings}, tween::Tween, LoopBehavior};
use winit::{event::VirtualKeyCode, window::CursorGrabMode};

impl engine_sdk::Engine for Engine {
    fn load_atlas(&mut self, id: u32, image: &DynamicImage, params:LoadAtlasParams) {
        self.load_queue_start_length += 1;
        self.load_queue.push_back(Load::Atlas { id: id, img: image.clone(), params });
       /* self.graphics.load_texture(id, image, params.atlas);
        self.textures.insert(
            id,
            TextureAtlas::new(id, Rc::new(image.clone()), params.atlas, params.editor_props),
        );*/
    }

    fn draw_scene(&mut self, camera: &engine_sdk::Camera, scene: &engine_sdk::registry::Registry) {
        self.scene_renderer.prepare(&mut self.graphics, camera, scene);
    }

    fn draw_rect(&mut self, params: DrawRectParams) {
        let atlas = self.graphics.get_atlas(params.pic);
        self.canvas.draw_rect(params, &atlas);
    }

    fn screen_size(&self) -> engine_sdk::glam::Vec2 {
        Vec2::new(
            self.graphics.config.width as f32,
            self.graphics.config.height as f32,
        )
    }

    fn draw_line(&mut self, params: engine_sdk::DrawLineParams) {
        self.canvas.draw_lines([params].into());
    }

    fn draw_text(&mut self, params: engine_sdk::DrawTextParams) {
        self.canvas.draw_text(params);
    }

    fn atlas(&self, id: &u32) -> Option<engine_sdk::TextureAtlas> {
        self.textures.get(id).cloned()
    }

    fn mouse_pos(&self) -> Vec2 {
        self.input.mouse_pos
    }

    fn mouse_down(&self, button: u8) -> bool {
        self.input.mouse_pressed[button as usize % 4]
    }

    
    fn mouse_just_released(&self, button:u8) -> bool {
        for b in self.input.mouse_just_released.iter() {
            if *b == button {
                return true;
            }
        }
        false
    }


    fn key_down(&self, key_code: VirtualKeyCode) -> bool {
        if let Some(k) = self.input.keys_pressed.get(&key_code) {
            return *k;
        }

        false
    }

    fn keys_just_pressed(&self) -> &[VirtualKeyCode] {
        &self.input.keys_just_pressed
    }

    fn egui(&self) -> &egui::Context {
        &self.egui_ctx
    }

    fn dt(&self) -> f32 {
        let dt = self.diagnostics.frame_time.as_secs_f32();
        if dt < 1.0 {
            return dt;
        }
        
        1.0
    }

    fn mouse_wheel_delta(&self) -> Vec2 {
        self.input.mouse_wheel_delta
    }

    fn atlases(&self) -> Vec<TextureAtlas> {
        let textures: Vec<TextureAtlas> = self
            .textures.values().cloned()
            .collect();
        textures
    }

    fn egui_texture(&mut self, id:&u32) -> Option<egui::TextureHandle> {
        if let Some(texture) = self.egui_textures.get(id) {
            return Some(texture.clone());
        }
        if let Some(texture) = self.textures.get(id) {
            let img = texture.image();
            let img = img.crop_imm(0, 0, texture.width(0), texture.height(0));
            let w = img.width() as usize;
            let h = img.height() as usize;
            let bytes = img.to_rgba8().to_vec();
            let texture = self.egui_ctx.load_texture(format!("{}", id), egui::ColorImage::from_rgba_unmultiplied([w,h], &bytes), egui::TextureOptions::NEAREST);
            self.egui_textures.insert(*id, texture.clone());

            return Some(texture);
        }

        None
    }

    fn push_event(&mut self, event:Event) {
        self.new_events.push(event);
    }

    fn set_cursor_grabbed(&mut self, visible:bool) {
        self.cursor_visible = visible;
        let window = self.window.borrow_mut();
        window.set_cursor_visible(visible);
        match visible {
            true => {
                let _ = window.set_cursor_grab(CursorGrabMode::None);
            },
            false => {
                let _ = window.set_cursor_grab(CursorGrabMode::Confined);
            },
        }
        
    }

    fn mouse_motion(&self) -> Vec2 {
        self.input.mouse_motion
    }

    fn cursor_grabbed(&self) -> bool {
        self.cursor_visible
    }

  /*  fn play_sound(&self, sound:u32, _volume:f32) {
        if let Some(sound_data) = self.static_sound_data.get(&sound) {
            if let Ok(mut audio_manager) = self.audio_manager.try_borrow_mut() {
                let _ = audio_manager.play(sound_data.clone());
            }
        }
    }*/

    fn load_sound(&mut self, sound:u32, bytes:&[u8]) {
        self.load_queue_start_length += 1;
        self.load_queue.push_back(Load::Sound { id: sound, bytes: Vec::from(bytes) });
        /*let vec = Vec::from(bytes);
        let cursor = Cursor::new(vec);
        let sound_data = StaticSoundData::from_cursor(cursor, StaticSoundSettings::default()).expect("failed to load sound data");
        self.static_sound_data.insert(sound, sound_data);*/
    }

    fn elapsed_ms(&self) -> u128 {
        self.start.elapsed().as_millis()
    }
/*
    fn play_music(&self, sound:u32) {
        if let Ok(handle) = self.music.try_borrow_mut().as_deref_mut() {
            if let Some(handle) = handle {
                let _ = handle.stop(Tween::default());
            }

            *handle = None;
        }

        if let Some(sound_data) = self.static_sound_data.get(&sound) {
            if let Ok(mut audio_manager) = self.audio_manager.try_borrow_mut() {
                let mut sound_data = sound_data.clone();
                sound_data.settings.loop_behavior = Some(LoopBehavior {
                    start_position:0.0
                });
                if let Ok(handle) = audio_manager.play(sound_data) {
                    *self.music.borrow_mut() = Some(handle);
                }
            }
        }
    }

    fn stop_music(&self) {
        if let Ok(handle) = self.music.try_borrow_mut().as_deref_mut() {
            if let Some(handle) = handle {
                let _ = handle.stop(Tween::default());
            }

            *handle = None;
        }
    }*/

    fn time(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }

    fn play_sounds(&mut self, registry:&mut Registry) {

        // stop sounds where no entity is present
        let mut rm = Vec::new();
        for (id, (_, handle)) in self.active_sounds.iter_mut() {
            if registry.entity(*id).is_none() {
                let _ = handle.stop(Tween::default());
                rm.push(*id);
            }
        }
        for id in rm.drain(..) {
            self.active_sounds.remove(&id);
        }

        // iterate through all sound emitter entities
        for (id, mut sound_emitter) in registry.components::<SoundEmitter>().iter_mut() {
            match self.active_sounds.get_mut(&id) {
                // if entity has an active handle
                // update handle and entity
                Some((prev_sound_emitter, handle)) => {
                    match handle.state() {
                        kira::sound::static_sound::PlaybackState::Stopped => {
                            // sound has stopped, despawn entity
                            registry.push(move |r|r.despawn(id));
                        },
                        _=>{
                            if *prev_sound_emitter == *sound_emitter {
                                // no change to entity, update entity to match handle
                                sound_emitter.position_secs = handle.position();
                                *prev_sound_emitter = sound_emitter.clone();
                                //dbg!(handle.position());
                            } else {
                                // entity was changed since last time
                                // remove handle and let it be re-created next tick
                                if let Some((_, mut handle)) = self.active_sounds.remove(&id) {
                                    let _ = handle.stop(Tween::default());
                                }
                            }
                        }
                    }
                },
                None => {
                    // no active handle, create one
                    if let Some(sound_data) = self.static_sound_data.get(&sound_emitter.sound) {
                        if let Ok(mut audio_manager) = self.audio_manager.try_borrow_mut() {
                            let mut sound_data = sound_data.clone();
                            if sound_emitter.loops {
                                sound_data.settings.loop_behavior = Some(LoopBehavior {
                                    start_position: 0.0,
                                });
                            }
                            if let Ok(mut handle) = audio_manager.play(sound_data.clone()) {
                                let _ = handle.seek_to(sound_emitter.position_secs);
                                self.active_sounds.insert(id, (sound_emitter.clone(), handle));
                            }
                        }
                    }
                },
            }
        }

        registry.execute();
    }

    fn editor<'a>(&'a mut self) -> Option<&'a mut dyn engine_sdk::Editor> {
        if let Some(editor) = self.editor.as_mut() {
            return Some(editor);
        }
        None
    }

    
}
