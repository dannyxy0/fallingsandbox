mod vertex;
mod wgpu_wrapper;

use crate::vertex::vertices_from_matrix;
use crate::wgpu_wrapper::WgpuWrapper;
use falling_sand::elements::sand::new_sand;
use falling_sand::elements::stone::new_stone;
use falling_sand::elements::water::new_water;
use falling_sand::simulation::Simulation;
use falling_sand::vector::Vector;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use web_time::{Duration, Instant};
use winit::event::{ElementState, Event, MouseButton, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::Key;
use winit::window::Window;

const TICK_SPEED: Duration = Duration::from_millis(10);

async fn run(event_loop: EventLoop<()>, window: Window) {
    let mut simulation = Simulation::new(100, 100);
    let mut wgpu = WgpuWrapper::new(&window).await.unwrap();

    let drawable_elements = [new_sand, new_water, new_stone];
    let mut drawable_index = 0usize;
    let mut last_tick = Instant::now();
    let mut drawing = false;
    let mut cursor_position = Vector::new(0, 0);
    let mut current_element = Some(drawable_elements[0]());

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
                let _ = simulation
                    .matrix
                    .set(cursor_position, current_element.clone());
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
                        cursor_position = Vector::new(
                            ((position.x / wgpu.config.width as f64)
                                * simulation.matrix.width() as f64)
                                as isize,
                            ((position.y / wgpu.config.height as f64)
                                * simulation.matrix.height() as f64)
                                as isize,
                        );
                    }
                    WindowEvent::KeyboardInput { event, .. } => {
                        if event.state == ElementState::Pressed && !event.repeat {
                            match event.logical_key.as_ref() {
                                Key::Character("j") => {
                                    drawable_index = drawable_index.saturating_sub(1);
                                    current_element = Some(drawable_elements[drawable_index]());
                                }
                                Key::Character("l") => {
                                    drawable_index =
                                        (drawable_index + 1).min(drawable_elements.len() - 1);
                                    current_element = Some(drawable_elements[drawable_index]());
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

#[allow(dead_code)]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
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
