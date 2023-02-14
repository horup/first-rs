#[derive(Clone, Debug)]
pub struct Atlas {
    pub columns:u8,
    pub rows:u8,
}

impl Default for Atlas {
    fn default() -> Self {
        Self { rows: 1, columns: 1 }
    }
}

impl Atlas {
    pub fn new(columns:u8, rows:u8) -> Self {
        Self { rows, columns }
    }
    pub fn w(&self) -> f32 {
        1.0 / self.columns as f32
    }

    pub fn h(&self) -> f32 {
        1.0 / self.rows as f32
    }

    pub fn u(&self, index:u16) -> [f32;2] {
        let x = index % self.columns as u16;
        let x = x as f32 / self.columns as f32;
        [x, x+self.w()]
    }

    pub fn v(&self, index:u16) -> [f32;2] {
        let y = index / self.columns as u16;
        let y = y as f32 / self.rows as f32;
        [y, y+self.h()]
    }
}