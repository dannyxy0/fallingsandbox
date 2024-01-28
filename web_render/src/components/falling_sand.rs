use falling_sand::elements::sand;
use falling_sand::matrix::Matrix;
use falling_sand::simulation::{Cell, Simulation};
use falling_sand::vector::Vector;
use leptos::html::Canvas;
use leptos::*;
use std::cell::RefCell;
use std::time::Duration;
use web_sys::wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

#[component]
pub fn FallingSand(width: usize, height: usize, tick_delay: Duration) -> impl IntoView {
    let simulation = RefCell::new(Simulation::new(width, height));
    let sand = Some(sand::new_sand());
    let _ = simulation
        .borrow_mut()
        .matrix
        .fill(Vector::new(32, 32), Vector::new(64, 64), sand);

    let canvas_ref = create_node_ref::<Canvas>();
    let _interval_handle = set_interval_with_handle(
        move || {
            let canvas_context = canvas_ref_to_context(&canvas_ref);
            simulation.borrow_mut().tick();
            render_matrix(&canvas_context, &simulation.borrow().matrix);
        },
        tick_delay,
    )
    .unwrap();

    view! {
        <canvas id="fs_canvas" _ref=canvas_ref width=width height=height/>
    }
}

fn canvas_ref_to_context(canvas_ref: &NodeRef<Canvas>) -> CanvasRenderingContext2d {
    let canvas_element = canvas_ref
        .get_untracked()
        .expect("canvas_ref should be in dom");
    let context = canvas_element
        .get_context("2d")
        .expect("canvas_element should have context")
        .expect("2d context should exist");
    context
        .dyn_into::<CanvasRenderingContext2d>()
        .expect("canvas_context should cast to CanvasRenderingContext2d")
}

fn render_matrix(canvas_context: &CanvasRenderingContext2d, matrix: &Matrix<Cell>) {
    let canvas = canvas_context
        .canvas()
        .expect("canvas_context should have canvas");
    let width = canvas.width() as f64 / matrix.width() as f64;
    let height = canvas.height() as f64 / matrix.height() as f64;

    canvas_context.begin_path();
    canvas_context.set_fill_style(&"rgba(255, 255, 255, 1)".into());
    canvas_context.fill_rect(0f64, 0f64, canvas.width() as f64, canvas.height() as f64);

    for i in 0..matrix.width() {
        for j in 0..matrix.height() {
            let cell = matrix
                .get(Vector::new(i as isize, j as isize))
                .expect("Position is in bounds");
            if let Some(element) = cell {
                let color = format!(
                    "rgba({}, {}, {}, {})",
                    element.properties.color().red,
                    element.properties.color().green,
                    element.properties.color().blue,
                    element.properties.color().alpha / 255
                );
                canvas_context.set_fill_style(&color.into());
                canvas_context.fill_rect(i as f64 * width, j as f64 * height, width, height);
            }
        }
    }

    canvas_context.close_path();
}
