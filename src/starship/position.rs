use crate::starship::direction::Direction;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Position {
    pub fn new() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    pub fn move_position(&mut self, direction: &Direction) {
        self.x += direction.x as i32;
        self.y += direction.y as i32;
        self.z += direction.z as i32;
    }
}
