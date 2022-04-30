pub struct Sprite {
    pub x: f64,
    pub y: f64,
    pub texture_id: usize,
    pub player_dist: f64,
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
}
