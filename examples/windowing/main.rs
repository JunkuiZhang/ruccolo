fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let window_builder = winit::window::WindowBuilder::new()
        .with_title("Hello, world!")
        .with_inner_size(winit::dpi::PhysicalSize::new(1280, 720))
        .with_enabled_buttons(
            winit::window::WindowButtons::CLOSE | winit::window::WindowButtons::MINIMIZE,
        )
        .with_resizable(false);

    println!("Window attr: {:#?}", window_builder.window_attributes());

    let _window = window_builder.build(&event_loop).unwrap();
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
