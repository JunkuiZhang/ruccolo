pub struct Editor {}

impl Editor {
    pub fn new() -> Self {
        Editor {}
    }

    pub fn run(&self) {
        println!("Editor is running!");
        crate::runtime::run();
    }
}
