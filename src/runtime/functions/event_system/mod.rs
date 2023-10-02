use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};

use super::{
    render_system::{fps_manager::FpsManager, RenderManager},
    scene_system::SceneManager,
    window_system::WindowManager,
};

#[inline]
pub fn handle_event(
    event: Event<'_, ()>,
    control_flow: &mut ControlFlow,
    fps_manager: &mut FpsManager,
    window_manager: &WindowManager,
    scene_manager: &SceneManager,
    render_manager: &RenderManager,
) {
    match event {
        Event::NewEvents(_) => {
            fps_manager.tick();
            if fps_manager.elapsed() >= 1.0 {
                fps_manager.update(std::time::Instant::now());
                println!("FPS: {}", fps_manager.get_fps());
            }
        }
        Event::WindowEvent { event, .. } => match event {
            // WindowEvent::Resized(_) => todo!(),
            // WindowEvent::Moved(_) => todo!(),
            WindowEvent::CloseRequested => control_flow.set_exit(),
            // WindowEvent::Destroyed => todo!(),
            // WindowEvent::DroppedFile(_) => todo!(),
            // WindowEvent::HoveredFile(_) => todo!(),
            // WindowEvent::HoveredFileCancelled => todo!(),
            // WindowEvent::ReceivedCharacter(_) => todo!(),
            // WindowEvent::Focused(_) => todo!(),
            WindowEvent::KeyboardInput {
                input:
                    winit::event::KeyboardInput {
                        state: winit::event::ElementState::Pressed,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => handle_keycode(keycode, control_flow, render_manager),
            // WindowEvent::ModifiersChanged(_) => todo!(),
            // WindowEvent::Ime(_) => todo!(),
            // WindowEvent::CursorMoved {
            //     device_id,
            //     position,
            //     modifiers,
            // } => todo!(),
            // WindowEvent::CursorEntered { device_id } => todo!(),
            // WindowEvent::CursorLeft { device_id } => todo!(),
            // WindowEvent::MouseWheel {
            //     device_id,
            //     delta,
            //     phase,
            //     modifiers,
            // } => todo!(),
            // WindowEvent::MouseInput {
            //     device_id,
            //     state,
            //     button,
            //     modifiers,
            // } => todo!(),
            // WindowEvent::TouchpadMagnify {
            //     device_id,
            //     delta,
            //     phase,
            // } => todo!(),
            // WindowEvent::SmartMagnify { device_id } => todo!(),
            // WindowEvent::TouchpadRotate {
            //     device_id,
            //     delta,
            //     phase,
            // } => todo!(),
            // WindowEvent::TouchpadPressure {
            //     device_id,
            //     pressure,
            //     stage,
            // } => todo!(),
            // WindowEvent::AxisMotion {
            //     device_id,
            //     axis,
            //     value,
            // } => todo!(),
            // WindowEvent::Touch(_) => todo!(),
            // WindowEvent::ScaleFactorChanged {
            //     scale_factor,
            //     new_inner_size,
            // } => todo!(),
            // WindowEvent::ThemeChanged(_) => todo!(),
            // WindowEvent::Occluded(_) => todo!(),
            _ => {}
        },
        // Event::DeviceEvent { device_id, event } => todo!(),
        // Event::UserEvent(_) => todo!(),
        // Event::Suspended => todo!(),
        // Event::Resumed => todo!(),
        Event::MainEventsCleared => {
            render_manager.tick(&scene_manager.render_queue, scene_manager.camera.get_mvp());
            profiling::finish_frame!();
        }
        // Event::RedrawRequested(_) => todo!(),
        // Event::RedrawEventsCleared => todo!(),
        // Event::LoopDestroyed => todo!(),
        _ => {}
    }
}

#[inline]
fn handle_keycode(
    key: VirtualKeyCode,
    control_flow: &mut ControlFlow,
    render_manager: &RenderManager,
) {
    match key {
        VirtualKeyCode::Escape => control_flow.set_exit(),
        VirtualKeyCode::R => println!("Report: {:?}", render_manager.report()),
        // movement
        VirtualKeyCode::W => println!("Report: {:?}", render_manager.report()),
        VirtualKeyCode::A => println!("Report: {:?}", render_manager.report()),
        VirtualKeyCode::S => println!("Report: {:?}", render_manager.report()),
        VirtualKeyCode::D => println!("Report: {:?}", render_manager.report()),
        _ => {}
    }
}
