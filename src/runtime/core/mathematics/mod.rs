pub mod array;
mod matrix;

pub const PI: f32 = 3.141592653;
pub type Matrix4 = matrix::Matrix<f32, 4>;
pub type Array4 = array::Array<f32>;

/// Right-hand axis
pub fn rotate_x(angle: u32) -> Matrix4 {
    // lookat x-minus
    let angle = angle as f32 / 180.0 * PI;

    Matrix4::new([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, angle.cos(), -angle.sin(), 0.0],
        [0.0, angle.sin(), angle.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn rotate_y(angle: u32) -> Matrix4 {
    let angle = angle as f32 / 180.0 * PI;

    Matrix4::new([
        [angle.cos(), 0.0, angle.sin(), 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-angle.sin(), 0.0, angle.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn rotate_z(angle: u32) -> Matrix4 {
    let angle = angle as f32 / 180.0 * PI;

    Matrix4::new([
        [angle.cos(), -angle.sin(), 0.0, 0.0],
        [angle.sin(), angle.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn rotate_around(axis: Array4, angle: u32) -> Matrix4 {
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
            one_cos_a * xy - z * sin_a,
            one_cos_a * xz + y * sin_a,
            0.0,
        ],
        [
            one_cos_a * xy + z * sin_a,
            cos_a + one_cos_a * y * y,
            one_cos_a * yz - x * sin_a,
            0.0,
        ],
        [
            one_cos_a * xz - y * sin_a,
            one_cos_a * yz + x * sin_a,
            cos_a + one_cos_a * z * z,
            0.0,
        ],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

// TODO: 四元数旋转
pub fn rotate(axis: Array4, angle: u32) -> Matrix4 {
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
            2.0 * x * y - 2.0 * w * z,
            2.0 * x * z + 2.0 * w * y,
            0.0,
        ],
        [
            2.0 * x * y + 2.0 * w * z,
            1.0 - 2.0 * x * x - 2.0 * z * z,
            2.0 * y * z - 2.0 * w * x,
            0.0,
        ],
        [
            2.0 * x * z - 2.0 * w * y,
            2.0 * y * z + 2.0 * w * x,
            1.0 - 2.0 * x * x - 2.0 * y * y,
            0.0,
        ],
        [0.0, 0.0, 0.0, 1.0],
    ])
}
