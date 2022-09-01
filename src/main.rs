#![warn(clippy::pedantic)]

mod food;
mod snake;

pub enum GameMode {
    Menu,
    Playing,
    End,
}

mod prelude {
    pub use crate::food::*;
    pub use crate::snake::*;
    pub use bracket_lib::prelude::*;
    pub use rand::prelude::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::GameMode;
}

use prelude::*;
pub struct State {
    snake: Snake,
    food: Food,
    mode: GameMode,
}

impl State {
    fn new() -> Self {
        Self {
            snake: Snake::new(),
            food: Food::new(),
            mode: GameMode::Menu,
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        match self.mode {
            GameMode::Menu => {
                self.snake.menu_render(ctx, &mut self.mode);
            }

            GameMode::End => {
                self.snake.end_state_render(ctx, &mut self.mode);
            }

            GameMode::Playing => {
                self.food.render(ctx);
                self.snake.render(ctx);
                self.snake.update(ctx, &mut self.food, &mut self.mode);
            }
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Matopeli.exe")
        .with_fps_cap(20.0)
        .build()?;

    register_palette_color("blue", RGB::named(BLUE));
    register_palette_color("pink", RGB::named(MAGENTA));

    main_loop(context, State::new())
}
