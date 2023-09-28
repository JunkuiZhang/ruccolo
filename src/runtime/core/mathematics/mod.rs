pub mod array;
mod matrix;

pub const PI: f32 = 3.141592653;
pub type Matrix4 = matrix::Matrix<f32, 4>;
pub type Array4 = array::Array<f32>;

/// Right-hand axis
pub fn rotate_x(angle: i32) -> Matrix4 {
    // lookat x-minus
    let angle = angle as f32 / 180.0 * PI;

    Matrix4::new([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, angle.cos(), angle.sin(), 0.0],
        [0.0, -angle.sin(), angle.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn rotate_y(angle: i32) -> Matrix4 {
    let angle = angle as f32 / 180.0 * PI;

    Matrix4::new([
        [angle.cos(), 0.0, -angle.sin(), 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [angle.sin(), 0.0, angle.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn rotate_z(angle: i32) -> Matrix4 {
    let angle = angle as f32 / 180.0 * PI;

    Matrix4::new([
        [angle.cos(), angle.sin(), 0.0, 0.0],
        [-angle.sin(), angle.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn rotate_around(axis: Array4, angle: i32) -> Matrix4 {
    // 罗德里格斯公式Rodrigues
    // games101:P4 21:10
    let angle = angle as f32 / 180.0 * PI;
    let cos_a = angle.cos();
    let one_cos_a = 1.0 - cos_a;
    let sin_a = angle.sin();
    let x = axis.0[0];
    let y = axis.0[1];
    let z = axis.0[2];
    let xy = x * y;
    let xz = x * z;
    let yz = y * z;

    Matrix4::new([
        [
            cos_a + one_cos_a * x * x,
            one_cos_a * xy + z * sin_a,
            one_cos_a * xz - y * sin_a,
            0.0,
        ],
        [
            one_cos_a * xy - z * sin_a,
            cos_a + one_cos_a * y * y,
            one_cos_a * yz + x * sin_a,
            0.0,
        ],
        [
            one_cos_a * xz + y * sin_a,
            one_cos_a * yz - x * sin_a,
            cos_a + one_cos_a * z * z,
            0.0,
        ],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

// TODO: 四元数旋转
pub fn rotate(axis: Array4, angle: i32) -> Matrix4 {
    let angle = angle as f32 / 360.0 * PI; // agnle / 2

    // https://www.zhihu.com/tardis/zm/art/78987582?source_id=1005
    // https://www.bilibili.com/video/BV1Lt411U7og
    // https://eater.net/quaternions/video/rotation
    let w = angle.cos();
    let x = angle.sin() * axis.0[0];
    let y = angle.sin() * axis.0[1];
    let z = angle.sin() * axis.0[2];

    Matrix4::new([
        [
            1.0 - 2.0 * y * y - 2.0 * z * z,
            2.0 * x * y + 2.0 * w * z,
            2.0 * x * z - 2.0 * w * y,
            0.0,
        ],
        [
            2.0 * x * y - 2.0 * w * z,
            1.0 - 2.0 * x * x - 2.0 * z * z,
            2.0 * y * z + 2.0 * w * x,
            0.0,
        ],
        [
            2.0 * x * z + 2.0 * w * y,
            2.0 * y * z - 2.0 * w * x,
            1.0 - 2.0 * x * x - 2.0 * y * y,
            0.0,
        ],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn cross(x: &Array4, y: &Array4) -> Array4 {
    return Array4::new([
        x.0[1] * y.0[2] - x.0[2] * y.0[1],
        x.0[2] * y.0[0] - x.0[0] * y.0[2],
        x.0[0] * y.0[1] - x.0[1] * y.0[0],
        0.0,
    ]);
}

impl Array4 {
    #[allow(dead_code)]
    pub fn almost_eq(&self, other: &Self) -> bool {
        for index in 0..4 {
            if !f32_eq(self.0[index], other.0[index]) {
                return false;
            }
        }
        return true;
    }

    pub fn normalize(&mut self) {
        let mut sum = 0.0;
        for index in 0..3 {
            sum += self.0[index] * self.0[index];
        }
        sum = sum.sqrt();
        for index in 0..3 {
            self.0[index] /= sum;
        }
    }
}

impl Matrix4 {
    pub fn trans(&self) -> Matrix4 {
        Matrix4::new([
            [
                self.0[0].0[0],
                self.0[1].0[0],
                self.0[2].0[0],
                self.0[3].0[0],
            ],
            [
                self.0[0].0[1],
                self.0[1].0[1],
                self.0[2].0[1],
                self.0[3].0[1],
            ],
            [
                self.0[0].0[2],
                self.0[1].0[2],
                self.0[2].0[2],
                self.0[3].0[2],
            ],
            [
                self.0[0].0[3],
                self.0[1].0[3],
                self.0[2].0[3],
                self.0[3].0[3],
            ],
        ])
    }

    pub fn almost_eq(&self, other: &Matrix4) -> bool {
        for index in 0..4 {
            if !self.0[index].almost_eq(&other.0[index]) {
                return false;
            }
        }

        return true;
    }
}

unsafe impl bytemuck::Pod for Array4 {}
unsafe impl bytemuck::Zeroable for Array4 {}
unsafe impl bytemuck::Pod for Matrix4 {}
unsafe impl bytemuck::Zeroable for Matrix4 {}

#[allow(dead_code)]
fn f32_eq(x: f32, y: f32) -> bool {
    let diff = if x < y { y - x } else { x - y };
    if diff < 1e-6 {
        true
    } else {
        false
    }
}

#[allow(unused_imports)]
mod test {
    use super::{cross, rotate, rotate_around, rotate_x, rotate_y, rotate_z, Array4, Matrix4};

    #[test]
    fn trans_tests() {
        let a = Matrix4::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let target = Matrix4::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        assert!(a.trans().almost_eq(&target));

        let a = Matrix4::new([
            [1.0, 1.0, 1.0, 1.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let target = Matrix4::new([
            [1.0, 0.0, 0.0, 0.0],
            [1.0, 1.0, 0.0, 0.0],
            [1.0, 0.0, 1.0, 0.0],
            [1.0, 0.0, 0.0, 1.0],
        ]);
        assert!(a.trans().almost_eq(&target));

        let a = rotate_x(45);
        let target = Matrix4::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let res = a.trans() * a;
        assert!(res.trans().almost_eq(&target));

        let a = rotate_y(45) * rotate_y(-45);
        let target = Matrix4::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        assert!(a.trans().almost_eq(&target));
    }

    #[test]
    fn rotate_tests() {
        // rotate_x
        let x = Array4::new([0.0, 1.0, 0.0, 0.0]);
        let rotate_mat = rotate_x(90);
        let res = rotate_mat * x;
        let target = Array4::new([0.0, 0.0, 1.0, 0.0]);
        assert!(res.almost_eq(&target), "Rotate x {:?}, {:?}", res, target);
        // rotate_y
        let x = Array4::new([0.0, 0.0, 1.0, 0.0]);
        let rotate_mat = rotate_y(90);
        let res = rotate_mat * x;
        let target = Array4::new([1.0, 0.0, 0.0, 0.0]);
        assert!(res.almost_eq(&target), "Rotate y {:?}, {:?}", res, target);
        // rotate_z
        let x = Array4::new([1.0, 0.0, 0.0, 0.0]);
        let rotate_mat = rotate_z(90);
        let res = rotate_mat * x;
        let target = Array4::new([0.0, 1.0, 0.0, 0.0]);
        assert!(res.almost_eq(&target), "Rotate z {:?}, {:?}", res, target);
        // rotate_around
        let x = Array4::new([1.0, 0.0, 0.0, 0.0]);
        let rotate_mat = rotate_around(Array4::new([0.0, 0.0, 1.0, 0.0]), 90);
        let res = rotate_mat * x;
        let target = Array4::new([0.0, 1.0, 0.0, 0.0]);
        assert!(
            res.almost_eq(&target),
            "Rotate around {:?}, {:?}",
            res,
            target
        );
        // rotate
        let x = Array4::new([1.0, 0.0, 0.0, 0.0]);
        let rotate_mat = rotate(Array4::new([0.0, 0.0, 1.0, 0.0]), 90);
        let res = rotate_mat * x;
        let target = Array4::new([0.0, 1.0, 0.0, 0.0]);
        assert!(res.almost_eq(&target), "Rotate {:?}, {:?}", res, target);
    }

    #[test]
    fn more_rotate_tests() {
        let x = Array4::new([1.0, 0.0, 0.0, 0.0]);
        let rotate_mat = rotate(Array4::new([1.0, 0.0, 0.0, 0.0]), 90);
        let res = rotate_mat * x;
        let target = Array4::new([1.0, 0.0, 0.0, 0.0]);
        assert!(res.almost_eq(&target), "Rotate #1 {:?}, {:?}", res, target);

        let x = Array4::new([1.0, 0.0, 0.0, 0.0]);
        let rotate_mat = rotate(Array4::new([0.0, 1.0, 0.0, 0.0]), 90);
        let res = rotate_mat * x;
        let target = Array4::new([0.0, 0.0, -1.0, 0.0]);
        assert!(res.almost_eq(&target), "Rotate #2 {:?}, {:?}", res, target);
    }

    #[test]
    fn normalize_tests() {
        let mut x = Array4::new([1.0, 0.0, 0.0, 0.0]);
        let target = Array4::new([1.0, 0.0, 0.0, 0.0]);
        x.normalize();
        assert!(
            x.almost_eq(&target),
            "Normalize tests: {:?}, {:?}",
            x,
            target
        );

        let mut x = Array4::new([1.0, 0.0, 0.0, 1.0]);
        let target = Array4::new([1.0, 0.0, 0.0, 1.0]);
        x.normalize();
        assert!(
            x.almost_eq(&target),
            "Normalize tests: {:?}, {:?}",
            x,
            target
        );

        let mut x = Array4::new([10.0, 0.0, 0.0, 1.0]);
        let target = Array4::new([1.0, 0.0, 0.0, 1.0]);
        x.normalize();
        assert!(
            x.almost_eq(&target),
            "Normalize tests: {:?}, {:?}",
            x,
            target
        );
    }

    #[test]
    fn cross_product() {
        let x = Array4::new([1.0, 0.0, 0.0, 0.0]);
        let y = Array4::new([0.0, 1.0, 0.0, 0.0]);
        let res = cross(&x, &y);
        let target = Array4::new([0.0, 0.0, 1.0, 0.0]);
        assert!(res.almost_eq(&target));
    }
}
