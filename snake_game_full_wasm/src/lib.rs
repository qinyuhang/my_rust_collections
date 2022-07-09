// TODO: add random to generate food position
//pub mod snake;
pub mod utils;

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use web_sys::console::{log, log_1};
use snake_game::snake::{SnakeGame, Direction};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
//#[cfg(feature = "wee_alloc")]
//#[global_allocator]
//static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn get_window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) -> i32 {
    get_window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK")
}

/// The start or the main fn in wasm
#[wasm_bindgen(start)]
pub fn start() {
    log_1(&JsValue::from_str("game start!"));

    utils::set_panic_hook();

    let tick_id = Rc::new(Cell::new(0));
    let window = web_sys::window().unwrap();
    let width = window.inner_width().unwrap().as_f64().unwrap() as usize;
    let height = window.inner_height().unwrap().as_f64().unwrap() as usize;
    let dpr = window.device_pixel_ratio();
    let scale = Rc::new(Cell::new(4 * dpr as usize));
    let (canvas_width, canvas_height) = (width / scale.get(), height / scale.get());

    let canvas: web_sys::HtmlCanvasElement = get_window()
        .document()
        .unwrap()
        .get_element_by_id("root")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    canvas.set_width((scale.get() * canvas_width) as u32);
    canvas.set_height((scale.get() * canvas_height) as u32);

    let context = Rc::new(
        canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap(),
    );
    let game = Rc::new(RefCell::new(SnakeGame::new(
        canvas_width,
        canvas_height,
    )));

    // resize and start

    game.borrow().set_scale(scale.get());

    let frame_fn = Rc::new(RefCell::new(None));
    let g = frame_fn.clone();

    // for frame_fn
    let tick_id_t = tick_id.clone();
    let game_t = game.clone();
    let context_t = context.clone();
    let scale_t = scale.clone();
    // use g and frame_fn two pointer to make self pointer
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        game_t.borrow().next_frame();
        let data = game_t.borrow().get_png_with_scale();
        let (canvas_width, canvas_height) =
            (game_t.borrow().width.get(), game_t.borrow().height.get());

        // log_1(&JsValue::from(format!(
        //     "in a render: data length: {}, width: {}, height: {}, game: {{ width: {}, height: {} }}",
        //     data.len(),
        //     canvas_width as u32,
        //     canvas_height as u32,
        //     game_t.borrow().width.get(),
        //     game_t.borrow().height.get(),
        // )));

        let img = web_sys::ImageData::new_with_u8_clamped_array(
            Clamped(&data),
            (canvas_width * scale_t.get()) as u32,
        )
        .unwrap();
        context_t.put_image_data(&img, 0.0, 0.0).unwrap();
        //        (*context_t).put_image_data(data, 0, 0);
        // TODO: use get_png_with_scale to putImage
        // Set the body's text content to how many times this
        // requestAnimationFrame callback has fired.
        // let text = format!(
        //     "requestAnimationFrame has been called times. fire tick_id: {:?}",
        //     tick_id_t
        // );
        // log_1(&JsValue::from(text));
        // body().set_text_content(Some(&text));

        // Schedule ourself for another requestAnimationFrame callback.
        tick_id_t.set(request_animation_frame(frame_fn.borrow().as_ref().unwrap()));
    }) as Box<dyn FnMut()>));

    // for resize_fn
    let tick_id_r = tick_id.clone();
    let frame_fn_r = g.clone();
    let game_r = game.clone();
    let scale_r = scale.clone();
    let resize_fn = Closure::wrap(Box::new(move || {
        // TODO: throttle resize
        log_1(&JsValue::from(format!(
            "on Resize, cancel animation: {:?}",
            tick_id_r
        )));
        get_window()
            .cancel_animation_frame(tick_id_r.get())
            .unwrap();

        // resize game
        let window = web_sys::window().unwrap();
        let width = window.inner_width().unwrap().as_f64().unwrap() as usize;
        let height = window.inner_height().unwrap().as_f64().unwrap() as usize;
        let (canvas_width, canvas_height) = (width / scale_r.get(), height / scale_r.get());

        game_r.borrow().resize(canvas_width, canvas_height);

        // restart loop
        tick_id_r.set(request_animation_frame(
            frame_fn_r.borrow().as_ref().unwrap(),
        ));
    }) as Box<dyn Fn()>);

    // for key_handler
    let tick_id_k = tick_id.clone();
    let frame_fn_k = g.clone();
    let game_k = game.clone();
    let key_handler = Closure::wrap(Box::new(move |e: web_sys::Event| {
        if let Some(key_e) = wasm_bindgen::JsCast::dyn_ref::<web_sys::KeyboardEvent>(&e) {
            let dir = match key_e.key_code() {
                // enter key
                13 => {
                    // restart game
                    game_k.borrow().restart();
                    None
                }
                // space key
                32 => {
                    // toggle_pause
                    game_k.borrow().toggle_pause();
                    // stop rAF
                    (tick_id_k.get() == -1)
                        .then(|| {
                            tick_id_k.set(request_animation_frame(
                                frame_fn_k.borrow().as_ref().unwrap(),
                            ));
                        })
                        .or_else(|| {
                            get_window()
                                .cancel_animation_frame(tick_id_k.get())
                                .unwrap();
                            tick_id_k.set(-1);
                            None
                        });
                    None
                    // if tick_id_k.get() == -1 {
                    //     tick_id_k.set(request_animation_frame(
                    //         frame_fn_k.borrow().as_ref().unwrap(),
                    //     ));
                    // } else {
                    //     get_window()
                    //         .cancel_animation_frame(tick_id_k.get())
                    //         .unwrap();
                    //     tick_id_k.set(-1);
                    // }
                    // None
                }
                // ArrowLeft or a
                37 | 65 => Some(Direction::Left),
                // ArrowUp 38 or w 87
                38 | 87 => Some(Direction::Up),
                // ArrowRight
                39 | 68 => Some(Direction::Right),
                // ArrowDown or s
                40 | 83 => Some(Direction::Down),
                r => {
                    log_1(&JsValue::from(r));
                    None
                }
            };
            if let Some(dir) = dir {
                game.borrow().change_direction(dir);
            }
        }
        log_1(&JsValue::from("on key down"));
        log_1(&JsValue::from("kkk"));
        log_1(&JsValue::from(e));
        println!();
    }) as Box<dyn Fn(_)>);

    window
        .add_event_listener_with_callback("resize", resize_fn.as_ref().unchecked_ref())
        .unwrap();
    window
        .add_event_listener_with_callback("keydown", key_handler.as_ref().unchecked_ref())
        .unwrap();

    tick_id.set(request_animation_frame(g.borrow().as_ref().unwrap()));

    // keep f live long
    resize_fn.forget();
    key_handler.forget();
}
