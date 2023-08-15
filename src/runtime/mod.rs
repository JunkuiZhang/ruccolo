use crate::runtime::functions::render_system::fps_manager;

use self::functions::{render_system::RenderManager, window_system::WindowManager};

mod core;
mod functions;
mod platforms;
mod resources;
mod tools;

pub fn run() {
    log::info!("Engine runtime started.");
    let event_loop = winit::event_loop::EventLoop::new();
    let window_manager = WindowManager::new(&event_loop);
    let render_manager = pollster::block_on(RenderManager::new(&window_manager.window));
    let mut fps_manager = fps_manager::FpsManager::new();

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
        winit::event::Event::MainEventsCleared => {
            fps_manager.tick();
            if fps_manager.last_update.elapsed().as_millis() >= 1000 {
                fps_manager.update(std::time::Instant::now());
                println!("FPS: {}", fps_manager.get_fps());
            }
            render_manager.tick();
            profiling::finish_frame!();
        }
        _ => {}
    });
}
