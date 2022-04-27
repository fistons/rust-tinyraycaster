pub struct Sprite {
    x: f64,
    y: f64,
    texture_id: usize,
    player_dist: f64,
}

impl Sprite {
    pub fn new(x: f64, y: f64, texture_id: usize, player_dist: f64) -> Self {
        Self {
            x,
            y,
            texture_id,
            player_dist,
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

    pub fn get_player_dist(&self) -> f64 {
        self.player_dist
    }

    pub fn set_player_dist(&mut self, dist: f64) {
        self.player_dist = dist;
    }
}
