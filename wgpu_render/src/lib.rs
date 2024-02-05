use bytemuck::{cast_slice, Pod, Zeroable};
use falling_sand::elements::sand::new_sand;
use falling_sand::matrix::Matrix;
use falling_sand::simulation::{Cell, Simulation};
use falling_sand::vector::Vector;
use log::info;
use std::borrow::Cow;
use std::mem::size_of;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
use web_time::{Duration, Instant};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::*;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
}

async fn run(event_loop: EventLoop<()>, window: Window) {
    let mut simulation = Simulation::new(100, 100);
    simulation
        .matrix
        .fill(Vector::new(5, 5), Vector::new(10, 10), Some(new_sand()))
        .expect("Fill is out of bounds");

    let mut size = window.inner_size();
    size.width = size.width.max(1);
    size.height = size.height.max(1);

    let instance = Instance::default();

    let surface = instance
        .create_surface(&window)
        .expect("Couldnt create surface from window");
    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            force_fallback_adapter: false,
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");

    // Create the logical device and command queue
    let (device, queue) = adapter
        .request_device(
            &DeviceDescriptor {
                label: None,
                required_features: Features::empty(),
                required_limits: Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    // Load the shaders from disk
    let shader = device.create_shader_module(ShaderModuleDescriptor {
        label: None,
        source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
    });

    let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    let vertex_buffer_layout = VertexBufferLayout {
        array_stride: size_of::<Vertex>() as BufferAddress,
        step_mode: VertexStepMode::Vertex,
        attributes: &vertex_attr_array![0 => Float32x2, 1 => Float32x4],
    };

    let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[vertex_buffer_layout],
        },
        fragment: Some(FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(swapchain_format.into())],
        }),
        primitive: PrimitiveState::default(),
        depth_stencil: None,
        multisample: MultisampleState::default(),
        multiview: None,
    });

    let mut config = surface
        .get_default_config(&adapter, size.width, size.height)
        .expect("Surface is not supported by the adapter");
    surface.configure(&device, &config);

    info!("wgpu initialized");

    let mut last_updated = Instant::now();
    const TICK_SPEED: Duration = Duration::from_millis(100);

    let window = &window;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop
        .run(move |event, target| {
            if last_updated.elapsed() >= TICK_SPEED {
                last_updated = Instant::now();
                simulation.tick();
                window.request_redraw();
            }

            if let Event::WindowEvent {
                window_id: _,
                event,
            } = event
            {
                match event {
                    WindowEvent::Resized(new_size) => {
                        // Reconfigure the surface with the new size
                        config.width = new_size.width.max(1);
                        config.height = new_size.height.max(1);
                        surface.configure(&device, &config);
                        // On macos the window needs to be redrawn manually after resizing
                        window.request_redraw();
                    }
                    WindowEvent::RedrawRequested => {
                        let vertices = vertices_from_matrix(&simulation.matrix);
                        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
                            label: None,
                            contents: cast_slice(vertices.as_slice()),
                            usage: BufferUsages::VERTEX,
                        });

                        let frame = surface
                            .get_current_texture()
                            .expect("Failed to acquire next swap chain texture");
                        let view = frame.texture.create_view(&TextureViewDescriptor::default());
                        let mut encoder = device
                            .create_command_encoder(&CommandEncoderDescriptor { label: None });
                        {
                            let mut render_pass =
                                encoder.begin_render_pass(&RenderPassDescriptor {
                                    label: None,
                                    color_attachments: &[Some(RenderPassColorAttachment {
                                        view: &view,
                                        resolve_target: None,
                                        ops: Operations {
                                            load: LoadOp::Clear(Color::WHITE),
                                            store: StoreOp::Store,
                                        },
                                    })],
                                    depth_stencil_attachment: None,
                                    timestamp_writes: None,
                                    occlusion_query_set: None,
                                });
                            render_pass.set_pipeline(&render_pipeline);
                            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                            render_pass.draw(0..vertices.len() as u32, 0..1);
                        }

                        queue.submit(Some(encoder.finish()));
                        frame.present();
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

fn vertices_from_matrix(matrix: &Matrix<Cell>) -> Vec<Vertex> {
    // clip_space ranges from -1 to 1, so we need to divide 2 by xy
    let spacing_x = 2.0 / matrix.width() as f32;
    let spacing_y = 2.0 / matrix.height() as f32;

    matrix
        .matrix
        .iter()
        .enumerate()
        .filter_map(|(i, cell)| {
            cell.as_ref().map(|element| {
                let c = element.properties.color();
                let color = [c.red, c.green, c.blue, c.alpha].map(|x| x as f32 / 255.0);

                // origin is the top left position of the rectangle we need to draw
                // position from 0 to 1
                let mut origin_x = (i % matrix.width()) as f32 / matrix.width() as f32;
                let mut origin_y = (i / matrix.height()) as f32 / matrix.height() as f32;
                // position from -1 to 1
                origin_x = origin_x * 2.0 - 1.0;
                origin_y = 1.0 - origin_y * 2.0;

                [
                    Vertex {
                        position: [origin_x, origin_y],
                        color,
                    },
                    Vertex {
                        position: [origin_x, origin_y - spacing_y],
                        color,
                    },
                    Vertex {
                        position: [origin_x + spacing_x, origin_y],
                        color,
                    },
                    Vertex {
                        position: [origin_x + spacing_x, origin_y],
                        color,
                    },
                    Vertex {
                        position: [origin_x, origin_y - spacing_y],
                        color,
                    },
                    Vertex {
                        position: [origin_x + spacing_x, origin_y - spacing_y],
                        color,
                    },
                ]
            })
        })
        .flatten()
        .collect()
}
