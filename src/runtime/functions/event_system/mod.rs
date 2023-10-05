use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};

use super::{
    render_system::{fps_manager::FpsManager, RenderManager},
    scene_system::{camera::MoveDirection, SceneManager},
    window_system::WindowManager,
};

#[inline]
pub fn handle_event(
    event: Event<'_, ()>,
    control_flow: &mut ControlFlow,
    fps_manager: &mut FpsManager,
    _window_manager: &WindowManager,
    scene_manager: &mut SceneManager,
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
            } => handle_keycode(
                fps_manager.get_delta_t(),
                keycode,
                control_flow,
                render_manager,
                scene_manager,
            ),
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
            render_manager.tick(scene_manager.camera.get_mvp());
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
    delta_t: f32,
    key: VirtualKeyCode,
    control_flow: &mut ControlFlow,
    render_manager: &RenderManager,
    scene_manager: &mut SceneManager,
) {
    match key {
        VirtualKeyCode::Escape => control_flow.set_exit(),
        VirtualKeyCode::R => println!("Report: {:?}", render_manager.report()),
        // movement
        VirtualKeyCode::W => scene_manager
            .camera
            .camera_move(MoveDirection::Forward, delta_t),
        VirtualKeyCode::A => scene_manager
            .camera
            .camera_move(MoveDirection::Left, delta_t),
        VirtualKeyCode::S => scene_manager
            .camera
            .camera_move(MoveDirection::Backward, delta_t),
        VirtualKeyCode::D => scene_manager
            .camera
            .camera_move(MoveDirection::Right, delta_t),
        _ => {}
    }
}
