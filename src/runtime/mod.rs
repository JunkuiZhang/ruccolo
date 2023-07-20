use self::functions::{render_system::RenderManager, window_system::WindowManager};

mod core;
mod functions;
mod platforms;
mod resources;
mod tools;

pub fn run() {
    println!("Runtime is running!");
    let event_loop = winit::event_loop::EventLoop::new();
    let window_manager = WindowManager::new(&event_loop);
    let render_manager = pollster::block_on(RenderManager::new(&window_manager.window));

    event_loop.run(move |event, _, control_flow| match event {
        winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::CloseRequested => control_flow.set_exit(),
            winit::event::WindowEvent::KeyboardInput {
                input:
                    winit::event::KeyboardInput {
                        state: winit::event::ElementState::Pressed,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => match keycode {
                winit::event::VirtualKeyCode::Escape => control_flow.set_exit(),
                winit::event::VirtualKeyCode::R => {
                    println!("Report: {:?}", render_manager.report())
                }
                _ => {}
            },
            _ => {}
        },
        _ => {}
    });
}
