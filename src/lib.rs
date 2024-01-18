use rand::Rng;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, CanvasRenderingContext2d};

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

type Point = (f64, f64);

fn draw_triangle(
    context: &CanvasRenderingContext2d,
    top: &Point,
    left: &Point,
    right: &Point,
    color: (u8, u8, u8),
) {
    context.begin_path();
    context.move_to(top.0, top.1);
    context.line_to(left.0, left.1);
    context.line_to(right.0, right.1);
    context.line_to(top.0, top.1);
    context.close_path();
    context.stroke();

    let color_str = format!("rgb({}, {}, {})", color.0, color.1, color.2);
    context.set_fill_style(&JsValue::from_str(&color_str));
    context.fill();
}

fn draw_sierpinski(
    context: &CanvasRenderingContext2d,
    top: &Point,
    left: &Point,
    right: &Point,
    color: (u8, u8, u8),
    depth: u32,
) {
    draw_triangle(context, top, left, right, color);
    if depth == 0 {
        return;
    }

    let left_mid = ((top.0 + left.0) / 2.0, (top.1 + left.1) / 2.0);
    let right_mid = ((top.0 + right.0) / 2.0, (top.1 + right.1) / 2.0);
    let bottom_mid = ((left.0 + right.0) / 2.0, (left.1 + right.1) / 2.0);

    let depth = depth - 1;

    let mut rng = rand::thread_rng();
    let next_color = (
        rng.gen_range(0..255),
        rng.gen_range(0..255),
        rng.gen_range(0..255),
    );

    draw_sierpinski(context, top, &left_mid, &right_mid, next_color, depth);
    draw_sierpinski(context, &left_mid, left, &bottom_mid, next_color, depth);
    draw_sierpinski(context, &right_mid, &bottom_mid, right, next_color, depth);
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_ts() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    console::log_1(&JsValue::from_str("draw triangle!"));

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let top = (300.0, 0.0);
    let left = (0.0, 600.0);
    let right = (600.0, 600.0);

    draw_sierpinski(&context, &top, &left, &right, (0, 255, 0), 6);

    Ok(())
}
