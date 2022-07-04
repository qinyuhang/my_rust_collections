use crate::random::random_range;
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub type Position = (usize, usize);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}

pub struct SnakeGame {
    pub width: Cell<usize>,
    pub height: Cell<usize>,
    pub direction: Cell<Direction>,
    next_direction: Cell<Direction>,
    // TODO: change speed to f32
    pub speed: Cell<usize>,
    pub snake: RefCell<VecDeque<Position>>,
    pub food: Cell<Position>,
    pub is_finished: Cell<bool>,
    pub scale: Cell<usize>,
    pub food_color: Cell<(u8, u8, u8)>,
    pub snake_color: Cell<(u8, u8, u8)>,
    pub frame_per_tick: Cell<usize>,
    frame_count_in_one_tick: Cell<usize>,
    pub is_paused: Cell<bool>,
}

impl SnakeGame {
    pub fn default() -> Self {
        let width = 10;
        let height = 10;
        let head_x = width / 2;
        let head_y = height / 2;
        let snake: Vec<Position> = (head_x..head_x + 3).map(|i| (i, head_y)).collect();
        let food = (random_range(0, width), random_range(0, height));
        SnakeGame {
            width: Cell::new(10),
            height: Cell::new(10),
            direction: Cell::new(Direction::Down),
            next_direction: Cell::new(Direction::Down),
            speed: Cell::new(1),
            snake: RefCell::new(VecDeque::from(snake)),
            food: Cell::new(food),
            is_finished: Cell::new(false),
            scale: Cell::new(10),
            food_color: Cell::new((0x00, 0xff, 0x00)),
            snake_color: Cell::new((0xbb, 0xbb, 0xbb)),
            frame_per_tick: Cell::new(10),
            frame_count_in_one_tick: Cell::new(0),
            is_paused: Cell::new(false),
        }
    }

    pub fn toggle_pause(&self) {
        self.is_paused.set(!self.is_paused.get());
    }

    #[allow(dead_code)]
    pub fn new(width: usize, height: usize) -> Self {
        let s = SnakeGame::default();
        s.resize(width, height);
        let head_x = width / 2;
        let snake: Vec<Position> = (head_x..head_x + 3).map(|i| (i, head_x)).collect();
        *(s.snake.borrow_mut()) = VecDeque::from(snake);
        let food = (random_range(0, width), random_range(0, height));
        s.food.set(food);
        s
    }

    pub fn restart(&self) {
        self.is_paused.set(true);
        self.move_snake_to_center();
        self.direction.set(Direction::Down);
        self.next_direction.set(Direction::Down);
        self.is_finished.set(false);
        self.is_paused.set(false);
    }

    pub fn resize(&self, width: usize, height: usize) {
        self.width.set(width);
        self.height.set(height);
    }

    pub fn move_snake_to_center(&self) {
        let head_x = self.width.get() / 2;
        let head_y = self.height.get() / 2;
        let snake: Vec<Position> = (head_x..head_x + 2).map(|i| (i, head_y)).collect();
        *(self.snake.borrow_mut()) = VecDeque::from(snake);
    }

    pub fn move_food_to_random(&self) {
        self.add_food();
    }

    #[allow(dead_code)]
    /// get the score of the snake
    pub fn score(&self) -> usize {
        self.snake.borrow().len()
    }

    pub fn change_direction(&self, dir: Direction) -> &Self {
        if self.is_finished.get() {
            return self;
        }
        match (self.direction.get(), dir) {
            (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up)
            | (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left) => {}
            _ => self.next_direction.set(dir),
        }
        self
    }

    pub fn is_position_valid(&self, p: Position) -> bool {
        p.0 >= 0 && p.1 >= 0 && p.0 < self.width.get() && p.1 < self.height.get()
    }

    /// the world or the time manager should call
    /// this function every frame
    /// etc: requestFrameAnimation(s.next_frame())
    pub fn next_frame(&self) -> &Self {
        let frame_count_in_one_tick = self.frame_count_in_one_tick.get();
        let frame_per_tick = self.frame_per_tick.get();
        if frame_count_in_one_tick < frame_per_tick {
            self.frame_count_in_one_tick
                .set(frame_count_in_one_tick + 1 % frame_per_tick);
            return self;
        }

        if self.is_paused.get() {
            return self;
        }
        if self.is_finished.get() {
            return self;
        }

        self.frame_count_in_one_tick.set(0);

        // update frame_per_tick

        self.direction.set(self.next_direction.get());

        let head = self.snake.borrow()[0];

        (1..=self.speed.get()).for_each(|step| {
            let p = match self.direction.get() {
                Direction::Up => (head.0, head.1 - step),
                Direction::Down => (head.0, head.1 + step),
                Direction::Left => (head.0 - step, head.1),
                Direction::Right => (head.0 + step, head.1),
            };
            if !self.is_position_valid(p) || self.snake.borrow().contains(&p) {
                log(&format!(
                    "set next {:?} is_position_valid: {:?}, contains: {:?}, snake: {:?}, head: {:?}",
                    p,
                    self.is_position_valid(p),
                    self.snake.borrow().contains(&p),
                    self.snake.borrow(),
                    head,
                ));
                self.is_finished.set(true);
                return;
            }
            if self.is_finished.get() {
                // log(&format!("set next {:?}", p));
                return;
            }

            self.snake.borrow_mut().push_front(p);
            // if meet food
            let food = self.food.get();
            (food != head).then(|| {
                self.snake.borrow_mut().pop_back();
            }).or_else(|| {  
                self.add_food();
                None
            });

            // if food != head {
            //     self.snake.borrow_mut().pop_back();
            // } else {
            //     // update food position
            //     self.add_food();
            //     // self.food.set((
            //     //     random_range(0, self.width.get()),
            //     //     random_range(0, self.height.get()),
            //     // ));
            //     // TODO: update speed
            //     // can just be 1 2 3 4, it will be a disaster
            //     // self.speed.set(self.snake.borrow().len() / 10 + 1);
            // }
            self.frame_per_tick.set(
                self.frame_per_tick.get() - self.snake.borrow().len() / 10
            );
        });
        self
    }

    fn add_food(&self) {
        loop {
            let food = (
                random_range(0, self.width.get()),
                random_range(0, self.height.get()),
            );
            if !self.snake.borrow().contains(&food) {
                self.food.set(food);
                break;
            }
        }
    }

    pub fn set_scale(&self, scale: usize) {
        self.scale.set(scale);
    }
    // no scale up
    pub fn get_png(&self) -> Vec<u8> {
        let pixel_count = self.width.get() * self.height.get();
        let pixel_len = 4;

        let mut buf = vec![0; pixel_count * pixel_len];

        // first fill rbga(0,0,0,255);
        buf.iter_mut().enumerate().for_each(|(idx, b)| {
            if idx % pixel_len == 3 {
                *b = 0xFF;
            }
        });

        // food with 0xffffff
        let food_r = (self.width.get() * self.food.get().1 + self.food.get().0) * pixel_len;
        (buf[food_r], buf[food_r + 1], buf[food_r + 2]) = self.food_color.get();

        // snake with 0x777777
        self.snake.borrow().iter().for_each(|block| {
            let pixel_r = (self.width.get() * block.1 + block.0) * 4;
            (buf[pixel_r], buf[pixel_r + 1], buf[pixel_r + 2]) = self.snake_color.get();
        });
        buf
        // png::Encoder::new(buf, self.width as u32, self.height as u32)
    }

    /// this function return the world in uint8Array, every u8 reprensent a pixel
    /// TODO: how to scale using self.scale
    /// 0,0 ->scale(10)-> (0,0 -> 9,9)
    /// 1,1 ->scale(10)-> (10,10 -> 19,19)
    /// x,y ->scale(10)-> (x*10,y*10 -> x*10+9,y*10+9)
    pub fn get_png_with_scale(&self) -> Vec<u8> {
        let scale = self.scale.get();
        let pixel_count = self.width.get() * self.height.get();
        let pixel_len = 4;

        let mut buf_after_scale = vec![0; pixel_count * pixel_len * scale * scale];

        // first fill rbga(0,0,0,255);
        buf_after_scale.iter_mut().enumerate().for_each(|(idx, b)| {
            if idx % pixel_len == 3 {
                *b = 0xFF;
            }
        });

        let food_position = self.food.get();
        for i in food_position.0 * scale..(food_position.0 * scale + scale) {
            for j in food_position.1 * scale..(food_position.1 * scale + scale) {
                // food with 0x00ff00
                let food_r = (self.width.get() * j * scale + i) * pixel_len;
                (
                    buf_after_scale[food_r],
                    buf_after_scale[food_r + 1],
                    buf_after_scale[food_r + 2],
                ) = self.food_color.get();
            }
        }

        // snake with 0x777777
        self.snake.borrow().iter().for_each(|block| {
            let food_position = block;
            for i in food_position.0 * scale..(food_position.0 * scale + scale) {
                for j in food_position.1 * scale..(food_position.1 * scale + scale) {
                    // food with 0x00ff00
                    let food_r = (self.width.get() * j * scale + i) * pixel_len;
                    (
                        buf_after_scale[food_r],
                        buf_after_scale[food_r + 1],
                        buf_after_scale[food_r + 2],
                    ) = self.snake_color.get();
                }
            }
        });
        buf_after_scale
    }
}
