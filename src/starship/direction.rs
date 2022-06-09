use bracket_geometry::prelude::Point3;

#[derive(Debug, Clone, Copy)]
pub struct Direction {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Direction {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn change_x(&mut self, delta: f32) {
        self.x += delta;
        if self.x > 1.0 {
            self.x = 1.0;
        }
        if self.x < -1.0 {
            self.x = -1.0;
        }
    }

    pub fn change_y(&mut self, delta: f32) {
        self.y += delta;
        if self.y > 1.0 {
            self.y = 1.0;
        }
        if self.y < -1.0 {
            self.y = -1.0;
        }
    }
}
