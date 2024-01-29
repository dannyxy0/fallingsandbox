use anyhow::{anyhow, Result};
use falling_sand::color::Color;
use falling_sand::matrix::Matrix;
use falling_sand::simulation::{Cell, Simulation};
use falling_sand::vector::Vector;
use leptos::html::Canvas;
use leptos::wasm_bindgen::JsValue;
use leptos::*;
use log::warn;
use std::cell::RefCell;
use std::time::Duration;
use web_sys::wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

#[component]
pub fn FallingSand(width: usize, height: usize, tick_delay: Duration) -> impl IntoView {
    let simulation = RefCell::new(Simulation::new(width, height));
    let canvas_ref = create_node_ref::<Canvas>();
    let canvas_stuff = move || resolve_canvas_stuff(canvas_ref());

    set_interval(
        move || {
            if let Some((canvas, context)) = canvas_stuff() {
                simulation.borrow_mut().tick();
                render_matrix(&canvas, &context, &simulation.borrow_mut().matrix);
            }
        },
        tick_delay,
    );

    view! {
        <canvas id="fs_canvas" _ref=canvas_ref width=width height=height/>
    }
}

fn render_matrix(
    canvas: &HtmlElement<Canvas>,
    render_context: &CanvasRenderingContext2d,
    matrix: &Matrix<Cell>,
) {
    let width = canvas.width() as f64 / matrix.width() as f64;
    let height = canvas.height() as f64 / matrix.height() as f64;

    // Reset canvas
    render_context.set_fill_style(&js_color(Color::white()));
    render_context.fill_rect(0f64, 0f64, canvas.width() as f64, canvas.height() as f64);

    // Draw matrix
    for i in 0..matrix.width() {
        for j in 0..matrix.height() {
            let cell = matrix
                .get(Vector::new_usize(i, j))
                .expect("pos is in bounds");
            if let Some(element) = cell {
                let color = js_color(element.properties.color());
                render_context.set_fill_style(&color);
                render_context.fill_rect(i as f64 * width, j as f64 * height, width, height);
            }
        }
    }
}

fn js_color(color: Color) -> JsValue {
    format!(
        "rgba({}, {}, {}, {})",
        color.red, color.green, color.blue, color.alpha
    )
    .into()
}

fn resolve_canvas_stuff(
    canvas_ref: Option<HtmlElement<Canvas>>,
) -> Option<(HtmlElement<Canvas>, CanvasRenderingContext2d)> {
    match canvas_ref_to_stuff(canvas_ref) {
        Ok(result) => Some(result),
        Err(error) => {
            warn!("{}", error);
            None
        }
    }
}

fn canvas_ref_to_stuff(
    canvas_ref: Option<HtmlElement<Canvas>>,
) -> Result<(HtmlElement<Canvas>, CanvasRenderingContext2d)> {
    let canvas = canvas_ref.ok_or(anyhow!("canvas_ref is none"))?;
    let context = canvas
        .get_context("2d")
        .map_err(|_| anyhow!("canvas type not set to 2d"))?
        .ok_or(anyhow!("no context exists"))?;
    let canvas_context = context
        .dyn_into::<CanvasRenderingContext2d>()
        .map_err(|_| anyhow!("could not cast context to 2d rendering context"))?;
    Ok((canvas, canvas_context))
}
