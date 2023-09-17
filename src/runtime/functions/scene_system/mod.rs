pub struct SceneManager {
    queue: Vec<()>,
}

impl SceneManager {
    pub fn new() -> Self {
        SceneManager { queue: Vec::new() }
    }
}
