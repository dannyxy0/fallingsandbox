mod vertex;
mod wgpu_wrapper;

use crate::vertex::vertices_from_matrix;
use crate::wgpu_wrapper::WgpuWrapper;
use falling_sand::elements::sand::new_sand;
use falling_sand::simulation::Simulation;
use falling_sand::vector::Vector;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

async fn run(event_loop: EventLoop<()>, window: Window) {
    let mut simulation = Simulation::new(100, 100);
    simulation
        .matrix
        .fill(Vector::new(10, 10), Vector::new(5, 5), Some(new_sand()))
        .unwrap();
    let mut wgpu = WgpuWrapper::new(&window).await.unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop
        .run(move |event, target| {
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
