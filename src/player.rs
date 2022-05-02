pub struct Player {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
    pub fov: f64,
    pub turn: i8,
    pub walk: i8,
}

impl Player {
    pub fn new(x: f64, y: f64, angle: f64, fov: f64) -> Self {
        Self {
            x,
            y,
            angle,
            fov,
            turn: 0,
            walk: 0,
        }
    }
}
