// TODO: add random to generate food position
mod random;
mod snake;
mod utils;

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

thread_local! {
    static GAME: Rc<RefCell<snake::SnakeGame>> = Rc::new(RefCell::new(snake::SnakeGame::default()))
}

#[wasm_bindgen]
pub fn generate_game(width: usize, height: usize) {
    GAME.with(|game| {
        game.borrow().resize(width, height);
        game.borrow().move_snake_to_center();
        game.borrow().move_food_to_random();
    })
}

#[wasm_bindgen]
pub fn handle_key(key: usize) {
    // 0 - ArrowUp
    // 1 - ArrowDown
    // 2 - ArrowLeft
    // 3 - ArrowRight
    // 4 - Space - toggle_pause
    // 5 - Enter - restart
    (match key {
        0 => Some(snake::Direction::Up),
        1 => Some(snake::Direction::Down),
        2 => Some(snake::Direction::Left),
        3 => Some(snake::Direction::Right),
        4 => {
            GAME.with(|game| {
                game.borrow()
                    .is_finished
                    .get()
                    .then(|| {
                        game.borrow().restart();
                    })
                    .or_else(|| {
                        game.borrow().toggle_pause();
                        None
                    });
            });
            None
        }
        5 => {
            GAME.with(|game| {
                game.borrow().restart();
            });
            None
        }
        // this will never happend
        _ => None,
    })
    .map(|direction| {
        GAME.with(|game| {
            game.borrow().change_direction(direction);
        });
    });
}

#[wasm_bindgen]
pub fn next_frame() {
    GAME.with(|game| {
        game.borrow().next_frame();
    });
}

#[wasm_bindgen]
pub fn set_scale(scale: usize) {
    GAME.with(|game| game.borrow().set_scale(scale));
}

#[wasm_bindgen]
pub fn get_png() -> Vec<u8> {
    let mut t = vec![];
    GAME.with(|game| {
        t = game.borrow().get_png();
    });
    t
}

#[wasm_bindgen]
pub fn get_png_with_scale() -> Vec<u8> {
    let mut t = vec![];
    GAME.with(|game| {
        t = game.borrow().get_png_with_scale();
    });
    t
}

#[wasm_bindgen]
pub fn resize(width: usize, height: usize) {
    GAME.with(|game| {
        game.borrow().resize(width, height);
    });
}
