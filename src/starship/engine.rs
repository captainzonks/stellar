#[derive(Debug, Clone, Copy)]
pub struct Engine {
    pub(crate) fuel: f32,
    pub engaged: bool,
}

impl Engine {
    pub(crate) fn new() -> Self {
        Self {
            fuel: 100.0,
            engaged: false,
        }
    }

    pub fn engage(&mut self) {
        self.engaged = true;
    }

    pub fn disengage(&mut self) {
        self.engaged = false;
    }

    pub fn burn(&mut self) {
        if self.fuel > 0.0 && self.engaged {
            self.fuel -= 1.0;
            println!("fuel remaining: {}", self.fuel);
        }
        if self.fuel <= 0.0 {
            self.disengage();
        }
    }
}
