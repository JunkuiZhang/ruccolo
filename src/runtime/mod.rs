use crate::runtime::functions::{
    event_system::handle_event, render_system::fps_manager, scene_system::SceneManager,
};

use self::functions::{render_system::RenderManager, window_system::WindowManager};

mod core;
mod functions;
mod platforms;
mod resources;
mod tools;

#[inline]
pub fn run() {
    log::info!("Engine runtime started.");
    let event_loop = winit::event_loop::EventLoop::new();
    let window_manager = WindowManager::new(&event_loop);
    let mut scene_manager = SceneManager::new();
    scene_manager.update();
    let render_manager = pollster::block_on(RenderManager::new(
        &window_manager.window,
        &scene_manager.camera,
    ));
    let mut fps_manager = fps_manager::FpsManager::new();

    event_loop.run(move |event, _, control_flow| {
        handle_event(
            event,
            control_flow,
            &mut fps_manager,
            &window_manager,
            &mut scene_manager,
            &render_manager,
        )
    });
}
