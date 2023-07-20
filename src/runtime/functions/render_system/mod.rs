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
}
