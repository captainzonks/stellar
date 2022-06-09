use crate::starship::direction::Direction;
use crate::starship::Starship;
use bracket_lib::prelude::VirtualKeyCode::*;
use bracket_lib::prelude::{BTerm, GameState, NAVY};

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

enum GameMode {
    MENU,
    PLAYING,
    END,
}

pub struct State {
    player: Starship,
    frame_time: f32,
    mode: GameMode,
}

impl State {
    pub fn new() -> Self {
        Self {
            player: Starship::new(),
            frame_time: 0.0,
            mode: GameMode::MENU,
        }
    }

    fn restart(&mut self) {
        self.player = Starship::new();
        self.frame_time = 0.0;
        self.mode = GameMode::PLAYING;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Stellar");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                P => self.restart(),
                Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                P => self.restart(),
                Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;

            if self.player.engine.engaged {
                self.player.engine.burn();
                self.player.position.move_position(&self.player.direction);
            }
        }
        self.player.render(ctx);

        // Start and Stop starship engines
        if let Some(E) = ctx.key {
            match self.player.engine.engaged {
                false => {
                    self.player.engage_engines(Direction::new());
                }
                true => {
                    self.player.disengage_engines();
                }
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::MENU => self.main_menu(ctx),
            GameMode::END => self.dead(ctx),
            GameMode::PLAYING => self.play(ctx),
        }
    }
}
