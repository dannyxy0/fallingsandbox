mod vertex;
mod wgpu_wrapper;

use crate::vertex::vertices_from_matrix;
use crate::wgpu_wrapper::WgpuWrapper;
use falling_sand::elements::element::Element;
use falling_sand::elements::sand::new_sand;
use falling_sand::elements::stone::new_stone;
use falling_sand::elements::water::new_water;
use falling_sand::idx;
use falling_sand::simulation::Simulation;
use lazy_static::lazy_static;
use nalgebra::Vector2;
use std::sync::Mutex;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use web_time::{Duration, Instant};
use winit::event::{ElementState, Event, MouseButton, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::Key;
use winit::window::Window;

const TICK_SPEED: Duration = Duration::from_millis(10);

// Game state
// Has to be a singleton to be accessible through a wasmbind function through js
lazy_static! {
    static ref CURRENT_ELEMENT: Mutex<Option<Element>> = Mutex::new(Some(DRAWABLE_ELEMENTS[0]()));
}
const DRAWABLE_ELEMENTS: [fn() -> Element; 3] = [new_sand, new_water, new_stone];

async fn run(event_loop: EventLoop<()>, window: Window) {
    let mut simulation = Simulation::new(100, 100);
    let mut wgpu = WgpuWrapper::new(&window).await.unwrap();

    let drawable_elements = DRAWABLE_ELEMENTS;
    let mut drawable_index = 0usize;
    let mut last_tick = Instant::now();
    let mut drawing = false;
    let mut cursor_position = Vector2::new(0, 0);

    let window = &window;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop
        .run(move |event, target| {
            if last_tick.elapsed() > TICK_SPEED {
                simulation.tick();
                last_tick = Instant::now();
                window.request_redraw();
            }
            if drawing {
                simulation.matrix[idx!(cursor_position)] = CURRENT_ELEMENT.lock().unwrap().clone();
                window.request_redraw();
            }

            if let Event::WindowEvent {
                window_id: _,
                event,
            } = event
            {
                match event {
                    WindowEvent::Resized(new_size) => wgpu.resize(new_size),
                    WindowEvent::RedrawRequested => {
                        let vertices = vertices_from_matrix(&simulation.matrix);
                        wgpu.render(vertices.as_slice()).unwrap()
                    }
                    WindowEvent::MouseInput { button, state, .. } => {
                        if button == MouseButton::Left {
                            drawing = state.is_pressed();
                        }
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        cursor_position = Vector2::new(
                            ((position.x / wgpu.config.width as f64)
                                * simulation.matrix.ncols() as f64)
                                as usize,
                            ((position.y / wgpu.config.height as f64)
                                * simulation.matrix.nrows() as f64)
                                as usize,
                        );
                    }
                    WindowEvent::KeyboardInput { event, .. } => {
                        if event.state == ElementState::Pressed && !event.repeat {
                            match event.logical_key.as_ref() {
                                Key::Character("j") => {
                                    drawable_index = drawable_index.saturating_sub(1);
                                    *(CURRENT_ELEMENT.lock().unwrap()) =
                                        Some(drawable_elements[drawable_index]());
                                }
                                Key::Character("l") => {
                                    drawable_index =
                                        (drawable_index + 1).min(drawable_elements.len() - 1);
                                    *(CURRENT_ELEMENT.lock().unwrap()) =
                                        Some(drawable_elements[drawable_index]());
                                }
                                _ => (),
                            }
                        }
                    }
                    WindowEvent::CloseRequested => target.exit(),
                    _ => (),
                }
            }
        })
        .unwrap();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn elements() -> Vec<String> {
    DRAWABLE_ELEMENTS
        .map(|x| x().properties.name().into())
        .to_vec()
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn set_current_element(element: &str) {
    *(CURRENT_ELEMENT.lock().unwrap()) = DRAWABLE_ELEMENTS
        .iter()
        .map(|x| x())
        .filter(|x| x.properties.name() == element)
        .next();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn set_current_element_delete() {
    *(CURRENT_ELEMENT.lock().unwrap()) = None;
}

#[allow(dead_code)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn start() {
    let event_loop = EventLoop::new().expect("Couldnt create event loop");
    #[allow(unused_mut)]
    let mut builder = winit::window::WindowBuilder::new();
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        use winit::platform::web::WindowBuilderExtWebSys;
        let canvas = web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.get_element_by_id("canvas"))
            .and_then(|cvs| cvs.dyn_into::<web_sys::HtmlCanvasElement>().ok())
            .expect("Couldnt find canvas in document");
        builder = builder.with_canvas(Some(canvas));
    }
    let window = builder
        .with_title("Falling Sand")
        .build(&event_loop)
        .expect("Couldnt build window");

    #[cfg(not(target_arch = "wasm32"))]
    env_logger::init();
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("Couldnt initialize logger");
    }
    pollster::block_on(run(event_loop, window));
}
