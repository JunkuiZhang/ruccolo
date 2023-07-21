pub mod fps_manager;

use crate::runtime::platforms::gpu::GpuContext;

pub struct RenderManager {
    gpu_context: GpuContext,
}

impl RenderManager {
    pub async fn new(window: &winit::window::Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::DX12,
            ..Default::default()
        });
        let surface = unsafe { instance.create_surface(window) }.unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        let surface_capabilities = surface.get_capabilities(&adapter);
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Primary Device"),
                    features: Default::default(),
                    limits: Default::default(),
                },
                None,
            )
            .await
            .unwrap();

        println!("Get adpter: {:#?}", adapter.get_info());

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_capabilities.formats[0],
            width: 1280,
            height: 720,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![surface_capabilities.formats[0]],
        };
        surface.configure(&device, &surface_config);

        RenderManager {
            gpu_context: GpuContext {
                instance,
                device,
                queue,
                surface,
                surface_config,
            },
        }
    }

    pub fn report(&self) {
        println!("Report: {:#?}", self.gpu_context.instance.generate_report());
    }

    pub fn tick(&self) {
        let frame = self.gpu_context.surface.get_current_texture().unwrap();
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
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
            let _ = command_encoder.begin_render_pass(&rp_desc);
        }

        // submit
        self.gpu_context
            .queue
            .submit(Some(command_encoder.finish()));

        frame.present();
    }
}
