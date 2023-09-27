use crate::runtime::core::mathematics::{Array4, Matrix4};

#[derive(Debug, Clone, Copy)]
pub struct CameraInfo {
    pub position: Array4,
    pub lookat: Array4,
    pub updir: Array4,
}

impl Default for CameraInfo {
    fn default() -> Self {
        Self {
            position: Array4::new([0.0; 4]),
            lookat: Array4::new([0.0, 0.0, -1.0, 0.0]),
            updir: Array4::new([0.0, 1.0, 0.0, 0.0]),
        }
    }
}

impl CameraInfo {
    pub fn get_mvp(&self) -> Matrix4 {
        return Matrix4::default();
    }
}
