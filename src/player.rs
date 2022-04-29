pub struct Player {
    x: f64,
    y: f64,
    angle: f64,
    fov: f64,
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

    pub fn update(&mut self) {
        self.angle += self.turn as f64 * 0.075; // TODO measure elapsed time and modify the speed accordingly
                                                // Also, it's buggy as fuck
        self.x += self.walk as f64 * self.angle.cos() * 0.05;
        self.y += self.walk as f64 * self.angle.sin() * 0.05;
    }
}
