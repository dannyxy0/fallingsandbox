use crate::vertex::Vertex;
use anyhow::{Context, Result};
use bytemuck::cast_slice;
use std::borrow::Cow;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::*;
use winit::dpi::PhysicalSize;

pub struct WgpuWrapper<'a> {
    pub surface: Surface<'a>,
    pub device: Device,
    pub queue: Queue,
    pub render_pipeline: RenderPipeline,
    pub config: SurfaceConfiguration,
    pub size: PhysicalSize<u32>,
}

impl<'a> WgpuWrapper<'a> {
    pub async fn new(window: &'a winit::window::Window) -> Result<Self> {
        let size = window.inner_size().max(PhysicalSize::new(1, 1));

        let instance = Instance::default();

        let surface = instance
            .create_surface(window)
            .context("Couldnt create surface from window")?;
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .context("Failed to find an appropriate adapter")?;

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
            .context("Failed to create device")?;

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

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
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

        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .context("Surface is not supported by the adapter")?;
        surface.configure(&device, &config);

        Ok(WgpuWrapper {
            surface,
            device,
            queue,
            render_pipeline,
            config,
            size,
        })
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size > PhysicalSize::new(0, 0) {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(&mut self, vertices: &[Vertex]) -> Result<()> {
        let vertex_buffer = self.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: cast_slice(vertices),
            usage: BufferUsages::VERTEX,
        });

        let frame = self
            .surface
            .get_current_texture()
            .context("Failed to acquire next swap chain texture")?;
        let view = frame.texture.create_view(&TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
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
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.draw(0..vertices.len() as _, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
        frame.present();

        Ok(())
    }
}
