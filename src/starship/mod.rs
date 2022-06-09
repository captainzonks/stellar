use crate::starship::direction::Direction;
use crate::starship::engine::Engine;
use crate::starship::position::Position;
use bracket_lib::prelude::BTerm;

pub mod direction;
pub mod engine;
pub mod position;

#[derive(Debug, Clone, Copy)]
pub struct Starship {
    pub engine: Engine,
    pub position: Position,
    pub direction: Direction,
}

impl Starship {
    pub fn new() -> Self {
        Self {
            engine: Engine::new(),
            position: Position::new(),
            direction: Direction::new(),
        }
    }

    pub fn engage_engines(&mut self, direction: Direction) {
        self.engine.engage();
    }

    pub fn disengage_engines(&mut self) {
        self.engine.disengage();
    }

    pub fn render(&mut self, ctx: &mut BTerm) {
        ctx.print_centered(
            1,
            "Fuel Remaining: ".to_string() + self.engine.fuel.to_string().as_str(),
        );
        ctx.print_centered(
            8,
            "Position: ".to_string()
                + self.position.x.to_string().as_str()
                + ", ".to_string().as_str()
                + self.position.y.to_string().as_str()
                + ", ".to_string().as_str()
                + self.position.z.to_string().as_str(),
        )
    }
}
