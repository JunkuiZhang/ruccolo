fn main() {
    let mut logger = env_logger::Builder::from_default_env();
    logger.target(env_logger::Target::Stdout);
    logger.filter_level(log::LevelFilter::Info);
    logger.init();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::DX12,
        dx12_shader_compiler: wgpu::Dx12Compiler::Dxc {
            // dxil_path: Some(std::path::PathBuf::from("./shared")),
            // dxc_path: Some(std::path::PathBuf::from("./shared")),
            dxil_path: Some(std::path::PathBuf::from("./shared/dxil.dll")),
            dxc_path: Some(std::path::PathBuf::from("./shared/dxcompiler.dll")),
        },
    });
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    let surface = unsafe { instance.create_surface(&window) }.unwrap();
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        compatible_surface: Some(&surface),
        ..Default::default()
    }))
    .unwrap();
    let (_device, _queue) =
        pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default(), None))
            .unwrap();

    event_loop.run(|event, _, control_flow| match event {
        winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::CloseRequested => control_flow.set_exit(),
            winit::event::WindowEvent::KeyboardInput {
                input:
                    winit::event::KeyboardInput {
                        state: winit::event::ElementState::Pressed,
                        virtual_keycode: code,
                        ..
                    },
                ..
            } => match code {
                Some(winit::event::VirtualKeyCode::Escape)
                | Some(winit::event::VirtualKeyCode::Q) => control_flow.set_exit(),
                _ => {}
            },
            _ => {}
        },
        _ => {}
    });
}
