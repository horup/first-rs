use glam::{Vec2};
use serde::{Serialize, Deserialize};





#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Grid<T:Clone + Default> {
    size:usize,
    cells:Vec<T>
}

impl<T> Default for Grid<T> where T:Default+Clone {
    fn default() -> Self {
        Self::new(64)
    }
}
#[allow(dead_code)]
pub struct Visit<'a, T> {
    pub index:(i32, i32),
    pub cell:&'a T,
    pub x:f32,
    pub y:f32,
    pub d:f32
}


impl<T> Grid<T> where T:Default+Clone {
    pub fn new(size:usize) -> Self {
        Self { 
            size,
            cells:vec![T::default();size * size]
         }
    }
    pub fn size(&self)->usize {
        self.size
    }

    pub fn get_mut(&mut self, (x, y):(i32, i32)) -> Option<&mut T> {
        let size = self.size;
        if x >= 0 && y >= 0 {
            let index = y as usize * size + x as usize;
            return self.cells.get_mut(index);
        }

        None
    }

    pub fn get(&self, (x,y):(i32, i32)) -> Option<&T> {
        let size = self.size;
        if x >= 0 && y >= 0 {
            let index = y as usize * size + x as usize;
            return self.cells.get(index);
        }

        None
    }

    pub fn for_each_mut(&mut self, mut f:impl FnMut(&mut T,(i32,i32))) {
        for y in 0..self.size as i32 {
            for x in 0..self.size as i32 {
                if let Some(t) = self.get_mut((x, y)) {
                    f(t, (x, y));
                }
            }
        }
    }

    pub fn for_each(&self, mut f:impl FnMut(&T,(i32,i32))) {
        for y in 0..self.size as i32 {
            for x in 0..self.size as i32 {
                if let Some(t) = self.get((x, y)) {
                    f(t, (x, y));
                }
            }
        }
    }

    pub fn astar<F:Fn((i32, i32), &T)->bool>(&self, start:(i32, i32), end:(i32, i32), visit:F) -> Option<Vec<(i32, i32)>> {
        let p = pathfinding::directed::astar::astar(&start, |(nx, ny)| {
            let (nx, ny) = (*nx, *ny);
            let mut vec:Vec<((i32, i32), i32)> = Vec::with_capacity(4);
            for p in [(nx - 1, ny), (nx + 1, ny), (nx, ny - 1), (nx, ny + 1)] {
                if let Some(tile) = self.get(p) {
                    if !visit(p, &tile) {
                        vec.push((p, 1));
                    }
                }
            }
            return vec;
        }, |(nx, ny)|{
            let (vx, vy) = ((nx - end.0).abs(), (ny - end.1).abs());
            return vx + vy;
        }, |n|{
            return n == &end;
        });
        if let Some((vec, _)) = p {
            return Some(vec);
        }

        None
    }

    pub fn cast_ray<F:FnMut(Visit<T>)->bool>(&self, start:Vec2, end:Vec2, mut f:F) {
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
        let v = end - start;
        let dir = v.normalize_or_zero();
        if dir.length() == 0.0 {
            return;
        }
        let (mut tile_x, dtile_x, mut dt_x, ddt_x) = get_helper(1.0, start.x, dir.x);
        let (mut tile_y, dtile_y, mut dt_y, ddt_y) = get_helper(1.0, start.y, dir.y);
    
        let mut t = 0.0;
        if dir.x*dir.x + dir.y*dir.y > 0.0 {
            loop {
                if v.length() < t {
                    break;
                }
                let index = (tile_x as i32, tile_y as i32);
                if let Some(cell) = self.get(index) {
                    if f(Visit {index:index, cell: cell, d:t, x:tile_x, y:tile_y }) {
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
}