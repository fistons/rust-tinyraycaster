pub struct Sprite {
    x: f64,
    y: f64,
    texture_id: usize,
}

impl Sprite {
    pub fn new(x: f64, y: f64, texture_id: usize) -> Self {
        Self {
            x,
            y,
            texture_id: texture_id,
        }
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn get_id(&self) -> usize {
      self.texture_id
    }
}
