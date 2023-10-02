use std::path::PathBuf;

use crate::runtime::core::mathematics::Array4;

use self::{camera::CameraInfo, models::load};

pub mod camera;
pub mod models;

pub enum SceneObject {
    Plane(PlaneInfo),
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PlaneInfo {
    pub position: Array4,
    pub direction: Array4,
    pub color: Array4,
}

pub struct SceneManager {
    pub camera: CameraInfo,
    pub render_queue: Vec<SceneObject>,
}

impl SceneManager {
    pub fn new() -> Self {
        let path = PathBuf::new();
        let path = path
            .join("assets")
            .join("scenes")
            .join("CornellBox-Original")
            .join("CornellBox-Original.obj");
        load(path);

        SceneManager {
            camera: CameraInfo::default(),
            render_queue: Vec::new(),
        }
    }

    #[inline]
    pub fn update(&mut self) {
        let plane_pos = Array4::new([0.0, 0.0, 0.0, 1.0]);
        let plane_dir = Array4::new([0.0, 1.0, 0.0, 0.0]);
        let plane_col = Array4::new([0.4, 0.7, 0.5, 1.0]);
        self.render_queue.push(SceneObject::Plane(PlaneInfo {
            position: plane_pos,
            direction: plane_dir,
            color: plane_col,
        }));
    }
}
