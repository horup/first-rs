use glam::{Vec3, IVec2, Vec2};
use parry2d::{bounding_volume::BoundingVolume, na::Isometry2};
use slotmap::new_key_type;
use crate::{Grid, Sprite, Entities, SpatialHashmap, Tile};

new_key_type! {pub struct SpriteId;}

pub struct World<'a> {
    spatial_hashmap:SpatialHashmap,
    sprites:&'a Entities<SpriteId, Sprite>,
    pub ceiling_texture:u32,
    pub floor_texture:u32,
    tilemap:&'a Grid<Tile>,
    potential_colliders:Vec<SpriteId>
}

#[derive(Default)]
pub struct Collision {
    pub other_entity:Option<SpriteId>,
    pub tile:Option<IVec2>
}


impl<'a> World<'a> {
    pub fn new(sprites:&'a Entities<SpriteId, Sprite>, grid:&'a Grid<Tile>) -> Self {
        Self {
            spatial_hashmap:SpatialHashmap::new(sprites),
            sprites,
            ceiling_texture: 0,
            floor_texture: 0,
            tilemap: grid,
            potential_colliders:Vec::with_capacity(64)
        }
    }

    pub fn query_around(&self, pos:Vec2, radius:f32, results:&mut Vec<SpriteId>) {
        self.spatial_hashmap.query_around(pos, radius, results);
    }

    pub fn sprites(&self) -> &'a Entities<SpriteId, Sprite> {
        self.sprites
    }

    pub fn tilemap(&self) -> &'a Grid<Tile> {
        self.tilemap
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
                                let d = e.pos - other_e.pos;
                                let r2 = e.radius + other_e.radius;
                                if d.length() < r2 {
                                    let r = r2 - d.length();
                                    let v = d.normalize() * r;
                                    pos_new += v.truncate().extend(0.0); // ignore z movement

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
                        self.spatial_hashmap.update_pos(id, e.pos.truncate());
                    }
                }
            }
        }
        
        col
    }
}

