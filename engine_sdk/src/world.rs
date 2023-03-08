use glam::{Vec3, IVec2, Vec2};
use parry2d::{bounding_volume::BoundingVolume, na::Isometry2};
use slotmap::new_key_type;
use crate::{Grid, Sprite, Entities, SpatialHashmap, Tile};

new_key_type! {pub struct SpriteId;}

pub struct World<'a> {
    spatial_hashmap:SpatialHashmap<'a>,
    sprites:&'a Entities<SpriteId, Sprite>,
    pub ceiling_texture:u32,
    pub floor_texture:u32,
    tilemap:&'a Grid<Tile>,
    potential_colliders:Vec<SpriteId>,
    collisions:Vec<Collision>
}

#[derive(Default)]
pub struct Collision {
    pub other_entity:Option<SpriteId>,
    pub tile:Option<IVec2>
}

pub struct Ray {
    pub start:Vec2,
    pub end:Vec2
}
#[allow(dead_code)]
pub struct Visit<'a> {
    pub tile:&'a Tile,
    pub x:f32,
    pub y:f32,
    pub d:f32
}

impl<'a> World<'a> {
    pub fn new(sprites:&'a Entities<SpriteId, Sprite>, grid:&'a Grid<Tile>) -> Self {
        Self {
            spatial_hashmap:SpatialHashmap::new(sprites),
            sprites,
            ceiling_texture: 0,
            floor_texture: 0,
            tilemap: grid,
            potential_colliders:Vec::with_capacity(64),
            collisions:Vec::with_capacity(64)
        }
    }

    pub fn query_around(&mut self, pos:Vec2, radius:f32, results:&mut Vec<SpriteId>) {
        self.spatial_hashmap.query_around(pos, radius, results);
    }

    pub fn sprites(&mut self) -> &'a Entities<SpriteId, Sprite> {
        self.spatial_hashmap.invalidate();
        self.sprites
    }

    pub fn tilemap(&self) -> &'a Grid<Tile> {
        self.tilemap
    }

    pub fn cast_ray<F:FnMut(Visit)->bool>(&self, ray:Ray, mut f:F) {
        fn get_helper(cell_size:f32, pos:f32, dir:f32) -> (f32, f32, f32, f32) {
            let tile = (pos / cell_size).floor();// + 1.0;
            let dtile;
            let dt;
            if dir > 0.0 {
                dtile = 1.0;
                dt = ((tile + 1.0) * cell_size - pos) / dir;
            } else {
                dtile = -1.0;
                dt = (tile  * cell_size - pos) / dir;
                // dt = ((tile + 1.0 ) * cell_size - pos) / dir;
            }
    
            (tile, dtile, dt, dtile * cell_size / dir)
        }
        let dir = (ray.end - ray.start).normalize_or_zero();
        if dir.length() == 0.0 {
            return;
        }
        let (mut tile_x, dtile_x, mut dt_x, ddt_x) = get_helper(1.0, ray.start.x, dir.x);
        let (mut tile_y, dtile_y, mut dt_y, ddt_y) = get_helper(1.0, ray.start.y, dir.y);
    
        let mut t = 0.0;
        if dir.x*dir.x + dir.y*dir.y > 0.0 {
            loop {
                if let Some(cell) = self.tilemap.get((tile_x as i32, tile_y as i32)) {
                    if f(Visit { tile: cell, d:t, x:tile_x, y:tile_y }) {
                        break;
                    }
                } else {
                    break;
                }
                if dt_x < dt_y {
                    tile_x += dtile_x;
                    let dt = dt_x;
                    t += dt;
                    dt_x = dt_x + ddt_x - dt;
                    dt_y -= dt;
                } else {
                    tile_y += dtile_y;
                    let dt = dt_y;
                    t += dt;
                    dt_x -= dt;
                    dt_y = dt_y + ddt_y - dt;
                }
            }
        } else {
        }
    }

    pub fn physics_step(&mut self, dt:f32) {
        self.spatial_hashmap.invalidate();
        for (id, sprite) in self.sprites.iter() {
            let new_pos = sprite.pos + sprite.vel * dt;
            self.clip_move(id, new_pos);
        }
    }

    pub fn clip_move(&mut self, id:SpriteId, new_pos:Vec3) -> Collision {
        let mut col = Collision::default();
        if let Some(e) = self.sprites.get_mut(id) {
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
                        self.spatial_hashmap.query_around(e.pos.truncate(), e.radius + v.length() + self.spatial_hashmap.max_radius(), &mut self.potential_colliders);
                        for other_id in self.potential_colliders.iter() {
                            let other_e = self.sprites.get(*other_id).unwrap();
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
                            if let Some(cell) = self.tilemap.get((np.x, np.y)) {
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
                        self.spatial_hashmap.update_one(id, e.pos.truncate());
                    }
                }
            }
        }
        col
    }
}

