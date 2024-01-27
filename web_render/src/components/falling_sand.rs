use leptos::*;
use leptos::html::Canvas;
use anyhow::Result;
use web_sys::CanvasRenderingContext2d;
use web_sys::wasm_bindgen::JsCast;
use falling_sand::matrix::{Matrix, Position};
use falling_sand::simulation::{Cell, Simulation};

#[component]
pub fn FallingSand(width: usize, height: usize) -> impl IntoView {
    let simulation = Simulation::new(width, height);

    let canvas_ref = create_node_ref::<Canvas>();
    create_effect(move |_| {
        let canvas_context = canvas_ref_to_context(&canvas_ref);
        let _ = render_matrix(&canvas_context, &simulation.matrix);
    });

    view! {
        <canvas id="fs_canvas" _ref=canvas_ref width=width height=height/>
    }
}

fn canvas_ref_to_context(canvas_ref: &NodeRef<Canvas>) -> CanvasRenderingContext2d {
    let canvas_element = canvas_ref.get_untracked()
        .expect("canvas_ref should be in dom");
    let context = canvas_element.get_context("2d")
        .expect("canvas_element should have context")
        .expect("2d context should exist");
    context.dyn_into::<CanvasRenderingContext2d>()
        .expect("canvas_context should cast to CanvasRenderingContext2d")
}

fn render_matrix(canvas_context: &CanvasRenderingContext2d, simulation: &Matrix<Cell>) -> Result<()> {
    let canvas = canvas_context.canvas().expect("canvas_context should have canvas");
    let width = canvas.width() as f64/simulation.width() as f64;
    let height = canvas.height() as f64/simulation.height() as f64;

    canvas_context.begin_path();
    canvas_context.set_fill_style(&"rgba(255, 0, 0, 1)".into());
    canvas_context.fill_rect(0f64, 0f64, canvas.width() as f64, canvas.height() as f64);

    for i in 0..simulation.width() {
        for j in 0..simulation.height() {
            if simulation.get(Position::new(i as isize, j as isize))?.is_some() {
                canvas_context.set_fill_style(&"rgba(0, 255, 0, 1)".into());
                canvas_context.fill_rect(i as f64 * width, j as f64 * height, width, height);
            }
        }
    }

    canvas_context.close_path();
    Ok(())
}
