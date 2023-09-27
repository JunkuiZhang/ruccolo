use crate::runtime::core::mathematics::{cross, Array4, Matrix4};

#[derive(Debug, Clone, Copy)]
pub struct CameraInfo {
    pub position: Array4,
    pub lookat: Array4,
    pub updir: Array4,
}

impl Default for CameraInfo {
    fn default() -> Self {
        Self {
            position: Array4::new([0.0, 0.0, 0.0, 1.0]),
            lookat: Array4::new([0.0, 0.0, -1.0, 0.0]),
            updir: Array4::new([0.0, 1.0, 0.0, 0.0]),
        }
    }
}

impl CameraInfo {
    pub fn get_mvp(&self) -> Matrix4 {
        let mut gt_perp = cross(&self.lookat, &self.updir);
        gt_perp.normalize();
        let move_transformation = Matrix4::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [
                -self.position.0[0],
                -self.position.0[1],
                -self.position.0[2],
                1.0,
            ],
        ]);
        let rotation_transform = Matrix4::new([
            [gt_perp.0[0], self.updir.0[0], -self.lookat.0[0], 0.0],
            [gt_perp.0[1], self.updir.0[1], -self.lookat.0[1], 0.0],
            [gt_perp.0[2], self.updir.0[2], -self.lookat.0[2], 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        return rotation_transform * move_transformation;
    }
}

#[allow(unused_imports)]
mod test {
    use crate::runtime::core::mathematics::Array4;

    use super::CameraInfo;

    #[test]
    fn camera_test() {
        let camera = CameraInfo::default();
        let transform = camera.get_mvp();
        let transed_pos = transform * camera.position;
        assert!(transed_pos.almost_eq(&camera.position));
        let transed_lookat = transform * camera.lookat;
        assert!(transed_lookat.almost_eq(&camera.lookat));
        let transed_updir = transform * camera.updir;
        assert!(transed_updir.almost_eq(&camera.updir));

        let mut camera = CameraInfo::default();
        camera.position = Array4::new([10.0, -20.0, 3.0, 1.0]);
        camera.lookat.normalize();
        camera.updir.normalize();
        let transform = camera.get_mvp();
        let transed_pos = transform * camera.position;
        assert!(transed_pos.almost_eq(&Array4::new([0.0, 0.0, 0.0, 1.0])));
        let transed_lookat = transform * camera.lookat;
        assert!(transed_lookat.almost_eq(&Array4::new([0.0, 0.0, -1.0, 0.0])));
        let transed_updir = transform * camera.updir;
        assert!(transed_updir.almost_eq(&Array4::new([0.0, 1.0, 0.0, 0.0])));
    }
}
