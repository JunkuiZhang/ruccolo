use self::camera::CameraInfo;

pub mod camera;

pub struct SceneManager {
    pub camera: CameraInfo,
    pub render_queue: Vec<()>,
}

impl SceneManager {
    pub fn new() -> Self {
        SceneManager {
            camera: CameraInfo::default(),
            render_queue: Vec::new(),
        }
    }
}
