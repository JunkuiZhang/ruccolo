pub mod fps_manager;

use wgpu::util::DeviceExt;

use crate::runtime::{core::mathematics::Matrix4, platforms::gpu::GpuContext};

use super::scene_system::{camera::CameraInfo, models::renderable::SceneRenderData, VerticesClip};

pub struct RenderManager<'a> {
    pub gpu_context: GpuContext,
    pipeline: wgpu::RenderPipeline,
    pub bindgroup: Vec<wgpu::BindGroup>,
    pub render_queue: Vec<SceneRenderData<'a>>,
}

const VERTICES: [[f32; 3]; 6] = [
    [-5.0, 0.0, -5.0],
    [5.0, 0.0, -5.0],
    [5.0, 0.0, -15.0],
    [5.0, 0.0, -15.0],
    [-5.0, 0.0, -15.0],
    [-5.0, 0.0, -5.0],
];

#[profiling::all_functions]
impl RenderManager<'_> {
    pub async fn new<'a>(window: &winit::window::Window, camera: &CameraInfo) -> RenderManager<'a> {
        let dxc_path = std::path::PathBuf::from("./shared");
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::DX12,
            dx12_shader_compiler: wgpu::Dx12Compiler::Dxc {
                dxil_path: Some(dxc_path.clone()),
                dxc_path: Some(dxc_path),
            },
        });
        let surface = unsafe { instance.create_surface(window) }.expect("Failed to create surface");
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");
        let surface_capabilities = surface.get_capabilities(&adapter);
        println!("Adapter features: {:#?}", adapter.features());
        println!("Adapter limitss: {:#?}", adapter.limits());
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Primary Device"),
                    features: wgpu::Features::PUSH_CONSTANTS,
                    limits: wgpu::Limits {
                        max_push_constant_size: 64,
                        ..Default::default()
                    },
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_capabilities.formats[0],
            width: 1280,
            height: 720,
            // present_mode: wgpu::PresentMode::AutoVsync,
            present_mode: wgpu::PresentMode::AutoNoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![surface_capabilities.formats[0]],
        };
        surface.configure(&device, &surface_config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Basic Shader"),
            source: wgpu::ShaderSource::Wgsl(
                include_str!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/assets/shaders/basic.wgsl"
                ))
                .into(),
            ),
        });

        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<[f32; 3]>() as _,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x3],
        };
        let bg_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bindgroup Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                // bind_group_layouts: &[&bg_layout],
                bind_group_layouts: &[],
                push_constant_ranges: &[wgpu::PushConstantRange {
                    stages: wgpu::ShaderStages::VERTEX,
                    range: 0..(std::mem::size_of::<Matrix4>() as u32),
                }],
            });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[vertex_buffer_layout],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        RenderManager {
            gpu_context: GpuContext {
                instance,
                device,
                queue,
                surface,
                surface_config,
            },
            pipeline,
            bindgroup: Vec::new(),
            render_queue: Vec::new(),
        }
    }

    #[profiling::skip]
    pub fn report(&self) {
        println!("Report: {:#?}", self.gpu_context.instance.generate_report());
    }

    #[inline]
    pub fn tick(&self, camera_mvp: Matrix4) {
        let frame = self.gpu_context.surface.get_current_texture().unwrap();
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor {
            // format: Some(self.gpu_context.surface_config.view_formats[0]),
            ..Default::default()
        });
        let rp_desc = wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: false,
                },
            })],
            depth_stencil_attachment: None,
        };

        let mut command_encoder = self
            .gpu_context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        // render pass
        {
            let mut pass = command_encoder.begin_render_pass(&rp_desc);
            pass.set_pipeline(&self.pipeline);
            pass.set_push_constants(
                wgpu::ShaderStages::VERTEX,
                0,
                bytemuck::cast_slice(&[camera_mvp]),
            );
            for renderable in self.render_queue.iter() {
                // pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
                // pass.draw(0..6, 0..1);
                pass.set_vertex_buffer(0, renderable.vertexbuffer.as_ref().unwrap().0.slice(..));
                pass.set_index_buffer(
                    renderable.indexbuffer.as_ref().unwrap().0.slice(..),
                    renderable.indexbuffer.as_ref().unwrap().1,
                );
                // pass.set_bind_group(0, &self.bindgroup[0], &[]);
                pass.draw_indexed(0..renderable.indexbuffer.as_ref().unwrap().2, 0, 0..1);
            }
            // pass.draw_indexed(indices, 0, 0..1);
        }

        // submit
        self.gpu_context
            .queue
            .submit(Some(command_encoder.finish()));

        frame.present();
    }
}
