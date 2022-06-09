#[derive(Debug, Clone, Copy)]
pub struct Direction {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl Direction {
    pub fn new() -> Self {
        Self { x: 1, y: 0, z: 0 }
    }
}
