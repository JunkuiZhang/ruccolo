pub struct FpsManager {
    delta_time: f32,
    fps: u32,
    frame_count: u32,
    last_update: std::time::Instant,
    last_frame: std::time::Instant,
}

impl FpsManager {
    pub fn new() -> Self {
        FpsManager {
            delta_time: 1.0 / 60.0,
            fps: 0,
            frame_count: 0,
            last_update: std::time::Instant::now(),
            last_frame: std::time::Instant::now(),
        }
    }

    #[inline]
    pub fn tick(&mut self) {
        self.frame_count += 1;
        self.delta_time = self.last_frame.elapsed().as_secs_f32();
        self.last_frame = std::time::Instant::now();
    }

    #[inline]
    pub fn get_fps(&self) -> u32 {
        self.fps
    }

    #[inline]
    pub fn get_delta_t(&self) -> f32 {
        self.delta_time
    }

    #[inline]
    pub fn update(&mut self, last_update: std::time::Instant) {
        self.fps = self.frame_count;
        self.frame_count = 0;
        self.last_update = last_update
    }

    #[inline]
    pub fn elapsed(&self) -> f32 {
        self.last_update.elapsed().as_secs_f32()
    }
}
