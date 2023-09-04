// TODO: add random to generate food position
//pub mod snake;
pub mod utils;

use snake_game::snake::{Direction, SnakeGame};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use web_sys::console::{log, log_1};
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
//#[cfg(feature = "wee_alloc")]
//#[global_allocator]
//static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const VERT_SHADER: &str = r#"//#version 300 es
// default value is vec2(0,0)
attribute vec2 position;
attribute vec4 color;

varying vec4 p_color;

uniform float pixel_scale;
void main() {
  p_color = color;
  float scale = pixel_scale;
  if (scale == 0.) {
     scale = 10.;
  }
  // move(-0.5, -0.5)
  vec2 tP = (position.xy - 0.5) * 2.;
  // reverse y axis
  tP.y = tP.y * -1.;
  gl_Position = vec4( tP, 0.0, 1.0 );
  gl_PointSize = scale;
}
"#;
const FRAG_SHADER: &str = r#"
precision highp float;
uniform vec2 resolution;
varying vec4 p_color; // food is green snake is gray

void main() {
  vec2 pixelPoxition = gl_FragCoord.xy / resolution;
  gl_FragColor = vec4(pixelPoxition, 0.0, 1.0);
  gl_FragColor = p_color;
  // gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0);
}
"#;

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
pub fn start() -> Result<(), JsValue> {
    log_1(&JsValue::from_str("game start!"));

    // log_1(&JsValue::from_str(&format!("{}", height)));
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
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap(),
    );
    let game = Rc::new(SnakeGame::new(canvas_width, canvas_height));
    // first clear the screen
    context.clear_color(0., 0., 0., 1.);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    let vert_shader = compile_shader(&context, WebGl2RenderingContext::VERTEX_SHADER, VERT_SHADER)?;
    let frag_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        FRAG_SHADER,
    )?;

    let program = link_program(&context, &vert_shader, &frag_shader).unwrap();
    context.use_program(Some(&program));

    let a_color = context.get_attrib_location(&program, "color");
    context.vertex_attrib4f(a_color as u32, 1., 1., 0., 1.);
    context.draw_arrays(WebGl2RenderingContext::POINTS, 0, 1);

    // resize and start
    // TODO: use rAF to call gl.draw_arrays

    game.set_scale(scale.get());

    let frame_fn = Rc::new(RefCell::new(None));
    let g = frame_fn.clone();

    // for frame_fn
    let tick_id_t = tick_id.clone();
    let game_t = game.clone();
    let context_t = context.clone();
    let scale_t = scale.clone();
    // use g and frame_fn two pointer to make self pointer
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        game_t.next_frame();

        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        let a_color = context_t.get_attrib_location(&program, "color");
        let a_position = context_t.get_attrib_location(&program, "position");
        // draw food
        context_t.vertex_attrib4f(a_color as u32, 0., 1., 0., 1.);
        let (width, height) = (game_t.width.get(), game_t.height.get());
        context_t.vertex_attrib2f(
            a_position as u32,
            game_t.food.get().0 as f32 / width as f32,
            game_t.food.get().1 as f32 / height as f32,
        );

        context_t.draw_arrays(WebGl2RenderingContext::POINTS, 0, 1);

        // TODO: get all points then draw food and snake

        game_t.snake.borrow().iter().for_each(|pt| {
            context_t.vertex_attrib2f(
                a_position as u32,
                pt.0 as f32 / width as f32,
                pt.1 as f32 / height as f32,
            );
            context_t.vertex_attrib4f(a_color as u32, 0.5, 0.5, 0.5, 1.);
            context_t.draw_arrays(WebGl2RenderingContext::POINTS, 0, 1);
        });
        // context_t.vertex_attrib4f(a_color as u32, 1., 1., 0., 1.);
        // context_t.draw_arrays(WebGl2RenderingContext::POINTS, 0, 1);
        // context_t.draw_arrays();

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

        game_r.resize(canvas_width, canvas_height);

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
                    game_k.restart();
                    None
                }
                // space key
                32 => {
                    // toggle_pause
                    game_k.toggle_pause();
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
                game.change_direction(dir);
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

    Ok(())
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}
