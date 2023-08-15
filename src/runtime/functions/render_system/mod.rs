pub mod fps_manager;

use crate::runtime::platforms::gpu::GpuContext;

pub struct RenderManager {
    gpu_context: GpuContext,
}

impl RenderManager {
    pub async fn new(window: &winit::window::Window) -> Self {
        // let dxc_path = std::path::PathBuf::from("./shared");
        let dxc_path = std::path::PathBuf::from("./shared/dxcompiler.dll");
        let dxil_path = std::path::PathBuf::from("./shared/dxil.dll");
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::DX12,
            dx12_shader_compiler: wgpu::Dx12Compiler::Dxc {
                // dxil_path: Some(dxc_path.clone()),
                // dxc_path: Some(dxc_path),
                dxil_path: Some(dxil_path),
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

    #[profiling::function]
    pub fn tick(&self) {
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
            let _ = command_encoder.begin_render_pass(&rp_desc);
        }

        // submit
        self.gpu_context
            .queue
            .submit(Some(command_encoder.finish()));

        frame.present();
    }
}
