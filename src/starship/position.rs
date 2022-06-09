use crate::starship::direction::Direction;
use bracket_geometry::prelude::Point3;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Position {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn move_position(&mut self, direction: &Direction) {
        self.x += direction.x;
        self.y += direction.y;

        let neg = if direction.y > 0.0 { 1.0 } else { -1.0 };
        self.z += (self.x * self.x + self.y * self.y).sqrt() * neg;
    }
}
