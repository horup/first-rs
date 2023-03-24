use glam::{Vec2, IVec2, Vec3};
use image::DynamicImage;
use parry2d::bounding_volume::BoundingVolume;
use parry2d::na::Isometry2;
use serde::{Serialize, Deserialize};
use winit::{event::VirtualKeyCode};
use world::EntityId;
use crate::world::World;
use crate::{Camera, Color, Event, Atlas, TextureAtlas, EditorProps, SpatialHashmap, Sprite, Grid, Tile};

#[derive(Default, Serialize, Deserialize, Clone, Copy)]
pub struct Collision {
    pub entity:EntityId,
    pub other_entity:Option<EntityId>,
    pub tile:Option<IVec2>
}


pub trait Engine {
    fn egui(&self) -> &egui::Context;
    fn egui_texture(&mut self, id:&u32) -> Option<egui::TextureHandle>;
    fn load_atlas(&mut self, id:u32, image:&DynamicImage, params:LoadAtlasParams);
    fn atlas(&self, id:&u32) -> Option<TextureAtlas>;
    fn atlases(&self) -> Vec<TextureAtlas>;
    fn draw_scene(&mut self, camera:&Camera, scene:&mut World);
    fn dt(&self) -> f32;
    fn draw_rect(&mut self, params:DrawRectParams);
    fn draw_line(&mut self, params:DrawLineParams);
    fn screen_size(&self) -> Vec2;
    fn draw_text(&mut self, params:DrawTextParams);
    fn mouse_pos(&self) -> Vec2;
    fn mouse_down(&self, button:u8) -> bool;
    fn mouse_wheel_delta(&self) -> Vec2;
    fn mouse_motion(&self) -> Vec2;
    fn key_down(&self, key_code:VirtualKeyCode) -> bool;
    fn keys_just_pressed(&self) -> &[VirtualKeyCode];
    fn push_event(&mut self, event:Event);
    fn key_just_pressed(&self, key_code:VirtualKeyCode) -> bool {
        self.keys_just_pressed().iter().any(|kc| kc == &key_code)
    }
    fn set_cursor_grabbed(&mut self, grabbed:bool);
    fn cursor_grabbed(&self) -> bool;

    fn physics_step(&mut self, world:&World, collisions:&mut Vec<Collision>)  {
        let dt = self.dt();
        collisions.clear();
        let mut spatial_hashmap = SpatialHashmap::new(world);
        let mut potential_colliders = Vec::with_capacity(1024);
        
        for e in world.entities() {
            if let Some(sprite) = e.get_mut::<Sprite>() {
                let new_pos = sprite.pos + sprite.vel * dt;
                let collision = self.clip_move(world, e.id(), new_pos, &mut spatial_hashmap, &mut potential_colliders);
                if collision.other_entity.is_some() || collision.tile.is_some() {
                    collisions.push(collision);
                }
            }
        }
    }

    fn clip_move(&mut self, world:&World, id:EntityId, new_pos:Vec3, spatial_hashmap:&mut SpatialHashmap, potential_colliders:&mut Vec<EntityId>) -> Collision {
        let mut col = Collision::default();
        let tilemap = world.singleton::<Grid<Tile>>().unwrap();
        col.entity = id;
        if let Some(e) = world.get_mut::<Sprite>(id) {
            let v = new_pos - e.pos;
            if v.length() > 0.0 {
                let mut left = v.length();
                let d = v.normalize();

                // FIXME: max step should be configurable at some point
                let max_step = 1.0 / 16.0;
                const DIMS: [Vec2; 2] = [Vec2::new(0.0, 1.0), Vec2::new(1.0, 0.0)];
                while left > 0.0 {
                    let mut step = left;
                    if step > max_step {
                        step = max_step;
                    }
                    let v = d * step;
                    left -= step;

                    for dim in DIMS {
                        let pos_org = e.pos;
                        let v = v.truncate() * dim;
                        if v.length() == 0.0 {
                            continue;
                        }

                        let mut pos_new = pos_org + v.extend(0.0);

                        // collision handling between entities
                        spatial_hashmap.query_around(e.pos.truncate(), e.radius + v.length() + spatial_hashmap.max_radius(),potential_colliders);
                        for other_id in potential_colliders.iter() {
                            let other_e = world.get::<Sprite>(*other_id).unwrap();
                            let ignore = !e.clips || !other_e.clips;
                            if *other_id != id && !ignore {
                                let s1_pos = Isometry2::translation(pos_new.x, pos_new.y);
                                let s1 = parry2d::shape::Ball::new(e.radius);
                                let aabb1 = s1.aabb(&s1_pos);
                                let s2_pos = Isometry2::translation(other_e.pos.x, other_e.pos.y);
                                let s2 = parry2d::shape::Ball::new(other_e.radius);
                                let aabb2 = s2.aabb(&s2_pos);

                                if aabb1.intersects(&aabb2) {
                                    pos_new = pos_org;
                                    
                                    // FIXME: last collision is saved, even though multiple might exist
                                    col.other_entity = Some(*other_id);
                                }
                            }
                        }

                        // collision between grid
                        let v = pos_new - pos_org;
                        let v = v.truncate() * dim;
                        let d = v.normalize();
                        let rev_dim = Vec2::new(dim.y, dim.x);
                        for i in [-1, 0, 1] {
                            let i = i as f32;
                            let cp = Vec2::new(i, i) * rev_dim + d + pos_org.truncate();
                            let np = cp.as_ivec2();
                            if let Some(cell) = tilemap.get((np.x, np.y)) {
                                if cell.clips {
                                    let s1 =
                                        parry2d::shape::Cuboid::new([e.radius, e.radius].into());
                                    let s1_pos = Isometry2::translation(pos_new.x, pos_new.y);
                                    let aabb1 = s1.aabb(&s1_pos);
                                    let s2 = parry2d::shape::Cuboid::new([0.5, 0.5].into());
                                    let s2_pos = Isometry2::translation(
                                        np.x as f32 + 0.5,
                                        np.y as f32 + 0.5,
                                    );
                                    let aabb2 = s2.aabb(&s2_pos);

                                    if aabb1.intersects(&aabb2) {
                                        pos_new = pos_org;

                                        col.tile = Some(np);
                                        break;
                                    }
                                }
                            }
                        }

                        e.pos = pos_new;
                        spatial_hashmap.update_one(id, e.pos.truncate());
                    }
                }
            }
        }
        col
    }


}


#[derive(Clone, Debug, Default)]
pub struct LoadAtlasParams {
    pub atlas:Atlas,
    pub editor_props:EditorProps
}

#[derive(Clone, Debug, Default)]
pub struct DrawRectParams {
    pub pos:Vec2,
    pub size:Vec2,
    pub color:Color,
    pub texture:Option<u32>,
    pub atlas_index:f32
}

#[derive(Clone, Debug, Default)]
pub struct DrawTextParams {
    pub screen_pos:Vec2,
    pub text:String,
    pub scale:f32,
    pub color:Color
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DrawLineParams {
    pub begin:Vec2,
    pub end:Vec2,
    pub line_width:f32,
    pub color:Color
}
