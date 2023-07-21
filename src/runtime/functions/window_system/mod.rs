pub struct WindowManager {
    pub window: winit::window::Window,
}

impl WindowManager {
    pub fn new(event_loop: &winit::event_loop::EventLoop<()>) -> Self {
        let window = winit::window::WindowBuilder::new()
            .with_title("Ruccolo Engine")
            .with_inner_size(winit::dpi::PhysicalSize::new(1280, 720))
            .with_enabled_buttons(
                winit::window::WindowButtons::CLOSE | winit::window::WindowButtons::MINIMIZE,
            )
            .with_resizable(false)
            // .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)))
            .build(event_loop)
            .unwrap();
        WindowManager { window }
    }
}
