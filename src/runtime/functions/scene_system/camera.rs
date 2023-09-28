use crate::runtime::core::mathematics::{cross, Array4, Matrix4};

#[derive(Debug, Clone, Copy)]
pub struct CameraInfo {
    pub position: Array4,
    pub lookat: Array4,
    pub updir: Array4,
    // half the total fov, current is height / 2 / znear
    fov2: f32,
    // camera pos to camera vision plane
    znear: f32,
    zfar: f32,
    aspect: f32, // width / height ie. 1080 / 1920
}

impl Default for CameraInfo {
    fn default() -> Self {
        Self {
            position: Array4::new([0.0, 5.0, 0.0, 1.0]),
            lookat: Array4::new([0.0, 0.0, -1.0, 0.0]),
            updir: Array4::new([0.0, 1.0, 0.0, 0.0]),
            fov2: 45.0,
            znear: 0.1,
            zfar: 1000.0,
            aspect: 9.0 / 16.0,
        }
    }
}

impl CameraInfo {
    pub fn modelview_transform_matrix(&self) -> Matrix4 {
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

    /// Here gives perspective projection matrix.
    pub fn projection_matrix(&self) -> Matrix4 {
        let scale = 1.0 / self.fov2.tan();
        let a = self.zfar / (self.znear - self.zfar);
        let b = self.znear * a;

        Matrix4::new([
            [self.aspect * scale, 0.0, 0.0, 0.0],
            [0.0, scale, 0.0, 0.0],
            [0.0, 0.0, a, -1.0],
            [0.0, 0.0, b, 0.0],
        ])
    }

    pub fn get_mvp(&self) -> Matrix4 {
        self.projection_matrix() * self.modelview_transform_matrix()
    }
}

#[allow(unused_imports)]
mod test {
    use crate::runtime::core::mathematics::Array4;

    use super::CameraInfo;

    #[test]
    fn camera_test() {
        let camera = CameraInfo::default();
        let transform = camera.modelview_transform_matrix();
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
        let transform = camera.modelview_transform_matrix();
        let transed_pos = transform * camera.position;
        assert!(transed_pos.almost_eq(&Array4::new([0.0, 0.0, 0.0, 1.0])));
        let transed_lookat = transform * camera.lookat;
        assert!(transed_lookat.almost_eq(&Array4::new([0.0, 0.0, -1.0, 0.0])));
        let transed_updir = transform * camera.updir;
        assert!(transed_updir.almost_eq(&Array4::new([0.0, 1.0, 0.0, 0.0])));
    }
}
