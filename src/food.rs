use crate::prelude::*;

pub struct Food {
    pub position: Point,
}

impl Food {
    pub fn new() -> Self {
        Self {
            position: Point::new(
                thread_rng().gen_range(0..SCREEN_WIDTH),
                thread_rng().gen_range(0..SCREEN_HEIGHT),
            ),
        }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(
            self.position.x,
            self.position.y,
            WHITE,
            BLACK,
            to_cp437('F'),
        )
    }

    fn overlap_with_snake(&self, point: Point, snake: &Snake) -> bool {
        let mut i = 0;
        let n = snake.snake_path.len() - 1;
        let mut found = false;
        loop {
            let curr_point = snake.snake_path[i as usize];

            if curr_point == point {
                found = true;
                break;
            }

            i += 1;

            if i == n {
                break;
            }
        }
        found
    }

    pub fn respawn(&mut self, snake: &Snake) {
        loop {
            let mut point = Point::new(
                thread_rng().gen_range(0..SCREEN_WIDTH),
                thread_rng().gen_range(0..SCREEN_HEIGHT),
            );

            if !self.overlap_with_snake(point, snake) {
                self.position = point;
                break;
            }
        }
    }
}
