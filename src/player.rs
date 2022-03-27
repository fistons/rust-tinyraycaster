pub struct Player {
    x: f64,
    y: f64,
    angle: f64,
    fov: f64,
}

impl Player {
    pub fn new(x: f64, y: f64, angle: f64, fov: f64) -> Self {
        Self { x, y, angle, fov }
    }

    pub fn get_pos(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    pub fn get_fov(&self) -> f64 {
        self.fov
    }

    pub fn get_angle(&self) -> f64 {
        self.angle
    }

    pub fn add_angle(&mut self, angle: f64) {
        self.angle += angle;
    }
}
