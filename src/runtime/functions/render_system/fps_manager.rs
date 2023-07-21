pub struct FpsManager {
    fps: u32,
    frame_count: u32,
    pub last_update: std::time::Instant,
}

impl FpsManager {
    pub fn new() -> Self {
        FpsManager {
            fps: 0,
            frame_count: 0,
            last_update: std::time::Instant::now(),
        }
    }

    pub fn tick(&mut self) {
        self.frame_count += 1;
    }

    pub fn get_fps(&self) -> u32 {
        self.fps
    }

    pub fn update(&mut self, last_update: std::time::Instant) {
        self.fps = self.frame_count;
        self.frame_count = 0;
        self.last_update = last_update
    }
}
