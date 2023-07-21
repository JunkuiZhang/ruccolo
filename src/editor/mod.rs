pub struct Editor {}

impl Editor {
    pub fn new() -> Self {
        Editor {}
    }

    pub fn run(&self) {
        log::info!("Engine editor started.");
        crate::runtime::run();
    }
}
