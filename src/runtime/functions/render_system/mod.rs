pub mod fps_manager;

use wgpu::util::DeviceExt;

use crate::runtime::{core::mathematics::Matrix4, platforms::gpu::GpuContext};

use super::scene_system::{camera::CameraInfo, SceneObject};

pub struct RenderManager {
    gpu_context: GpuContext,
    pipeline: wgpu::RenderPipeline,
    camera_trans_matrix: Matrix4,
    vertex_buffer: wgpu::Buffer,
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
impl RenderManager {
    pub async fn new(window: &winit::window::Window, camera: &CameraInfo) -> Self {
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

        let camera_uniforms = camera.get_mvp();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[VERTICES]),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let vertex_buffer_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<[f32; 3]>() as _,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &wgpu::vertex_attr_array![0 => Float32x3],
        };

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[wgpu::PushConstantRange {
                    stages: wgpu::ShaderStages::VERTEX,
                    range: 0..64,
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
            camera_trans_matrix: camera_uniforms,
            vertex_buffer,
        }
    }

    #[profiling::skip]
    pub fn report(&self) {
        println!("Report: {:#?}", self.gpu_context.instance.generate_report());
    }

    pub fn tick(&self, render_queue: &Vec<SceneObject>) {
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
                bytemuck::cast_slice(&[self.camera_trans_matrix]),
            );
            pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            pass.draw(0..6, 0..1);
            // pass.draw_indexed(indices, 0, 0..1);
        }

        // submit
        self.gpu_context
            .queue
            .submit(Some(command_encoder.finish()));

        frame.present();
    }
}
