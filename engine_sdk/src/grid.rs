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

    pub fn get_mut(&mut self, x:i32, y:i32) -> Option<&mut T> {
        let size = self.size;
        if x >= 0 && y >= 0 {
            let index = y as usize * size + x as usize;
            return self.cells.get_mut(index);
        }

        None
    }

    pub fn get(&self,x:i32, y:i32) -> Option<&T> {
        let size = self.size;
        if x >= 0 && y >= 0 {
            let index = y as usize * size + x as usize;
            return self.cells.get(index);
        }

        None
    }

    pub fn for_each_mut(&mut self, mut f:impl FnMut(&mut T,i32,i32)) {
        for y in 0..self.size as i32 {
            for x in 0..self.size as i32 {
                if let Some(t) = self.get_mut(x, y) {
                    f(t, x, y);
                }
            }
        }
    }

    pub fn for_each(&self, mut f:impl FnMut(&T,i32,i32)) {
        for y in 0..self.size as i32 {
            for x in 0..self.size as i32 {
                if let Some(t) = self.get(x, y) {
                    f(t, x, y);
                }
            }
        }
    }
}