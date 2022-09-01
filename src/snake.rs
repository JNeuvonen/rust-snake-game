use std::string;

use crate::{food, prelude::*};

pub struct Snake {
    pub snake_path: Vec<Point>,
    pub direction: Point,
    pub score: u32,
    pub body_char: char,
    pub bg: (u8, u8, u8),
    pub new_bg_frames_left: i8,
}

pub fn dir_left() -> Point {
    Point::new(-1, 0)
}

pub fn dir_right() -> Point {
    Point::new(1, 0)
}

pub fn dir_up() -> Point {
    Point::new(0, -1)
}

pub fn dir_down() -> Point {
    Point::new(0, 1)
}

impl Snake {
    pub fn new() -> Self {
        Self {
            snake_path: vec![Point::zero(), Point::new(1, 0), Point::new(2, 0)],
            direction: Point::new(1, 0),
            score: 0,
            bg: BLACK,
            new_bg_frames_left: 0,
            body_char: '*',
        }
    }

    fn get_snake_head(&self) -> Point {
        self.snake_path[self.snake_path.len() - 1]
    }

    fn restart(&mut self) {
        self.snake_path = vec![Point::zero(), Point::new(1, 0), Point::new(2, 0)];
        self.direction = Point::new(1, 0);
        self.score = 0;
        self.bg = BLACK;
        self.new_bg_frames_left = 0;
        self.body_char = '*';
    }

    pub fn render(&self, ctx: &mut BTerm) {
        let mut i = 0;
        let n = self.snake_path.len() - 1;
        loop {
            let point = &self.snake_path[i as usize];
            ctx.set(point.x, point.y, WHITE, self.bg, to_cp437(self.body_char));
            i += 1;

            if i == n {
                let point = &self.snake_path[(i) as usize];
                ctx.set(
                    point.x,
                    point.y,
                    WHITE,
                    self.bg,
                    to_cp437(self.get_head_char()),
                );

                break;
            }
        }
    }

    fn overlap(&self, point: Point) -> bool {
        let mut i = 0;
        let n = self.snake_path.len() - 1;
        let mut found = false;
        loop {
            let p = self.snake_path[i];

            if p == point {
                found = true;
                break;
            }

            i += 1;

            if i == n {
                break;
            }
        }
        return found;
    }

    fn legal_move(&self, point: Point) -> bool {
        let left = dir_left();
        let right = dir_right();
        let up = dir_up();
        let down = dir_down();

        let mut legal = true;

        if point == left && self.direction == right {
            legal = false;
        }

        if point == right && self.direction == left {
            legal = false;
        }

        if point == up && self.direction == down {
            legal = false;
        }

        if point == down && self.direction == up {
            legal = false;
        }

        legal
    }

    fn can_enter(&self, point: Point) -> bool {
        point.x >= 0
            && point.x < SCREEN_WIDTH
            && point.y >= 0
            && point.y < SCREEN_HEIGHT
            && !self.overlap(point)
    }

    fn compute_new_position(&mut self, new_head_pos: Point, food_tile: bool) {
        let mut i = 0;
        let n = self.snake_path.len() - 1;

        if food_tile {
            self.snake_path.push(self.snake_path[0]);
            let mut j = self.snake_path.len() - 1;
            loop {
                let helper = self.snake_path[j - 1];
                self.snake_path[j - 1] = self.snake_path[j];
                self.snake_path[j] = helper;

                j -= 1;
                if j == 0 {
                    break;
                }
            }
        } else {
            loop {
                if i == n {
                    self.snake_path[n] = new_head_pos;
                    break;
                }

                self.snake_path[i] = self.snake_path[i + 1];
                i += 1;
            }
        }
    }

    fn entered_food_tile(&mut self, point: Point, food: &mut Food) -> bool {
        if point == food.position {
            food.respawn(self);
            self.bg = GREENYELLOW;
            self.new_bg_frames_left = 15;
            self.body_char = 'X';
            return true;
        } else {
            if self.new_bg_frames_left == 0 {
                self.bg = BLACK;
                self.body_char = '*';
            } else {
                self.new_bg_frames_left -= 1;
            }
        }

        false
    }

    pub fn menu_render(&mut self, ctx: &mut BTerm, mode: &mut GameMode) {
        ctx.cls();
        ctx.print_centered(5, "Main menu");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => {
                    *mode = GameMode::Playing;
                }
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    pub fn end_state_render(&mut self, ctx: &mut BTerm, mode: &mut GameMode) {
        ctx.cls();
        ctx.print_centered(5, "Thanks for playing!");
        ctx.print_centered(6, &format!("You earned {} points", self.score));

        ctx.print_centered(8, "(P) Play again");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => {
                    self.restart();
                    *mode = GameMode::Playing;
                }
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn get_head_char(&self) -> char {
        if self.direction == dir_left() {
            return '<';
        }

        if self.direction == dir_right() {
            return '>';
        }

        if self.direction == dir_down() {
            return 'v';
        }

        if self.direction == dir_up() {
            return '^';
        }

        return 'H';
    }

    pub fn update(&mut self, ctx: &mut BTerm, food: &mut Food, mode: &mut GameMode) {
        if let Some(key) = ctx.key {
            let pos_delta: Point = match key {
                VirtualKeyCode::Left => dir_left(),
                VirtualKeyCode::Right => dir_right(),
                VirtualKeyCode::Up => dir_up(),
                VirtualKeyCode::Down => dir_down(),
                _ => self.direction,
            };

            if self.legal_move(pos_delta) {
                self.direction = pos_delta;
            }
        }

        let new_snake_head = self.get_snake_head() + self.direction;

        if self.can_enter(new_snake_head) {
            let food_tile = self.entered_food_tile(self.get_snake_head(), food);

            if food_tile {
                self.score += 1;
            }

            self.compute_new_position(new_snake_head, food_tile);
        } else {
            *mode = GameMode::End;
        }
    }
}
