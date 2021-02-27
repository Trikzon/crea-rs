use super::*;

#[derive(Copy, Clone, PartialEq)]
pub struct Matrix4(pub [f32; 4 * 4]);

impl Matrix4 {
    pub fn new(values: [f32; 4 * 4]) -> Matrix4 {
        Matrix4(values)
    }

    pub fn zero() -> Self {
        Matrix4([0.0; 4 * 4])
    }

    pub fn identity() -> Self {
        let mut matrix = Self::zero();

        matrix.set_value(0, 0, 1.0);
        matrix.set_value(1, 1, 1.0);
        matrix.set_value(2, 2, 1.0);
        matrix.set_value(3, 3, 1.0);

        matrix
    }

    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let mut matrix = Self::identity();

        matrix.set_value(0, 0, 2.0 / (right - left));
        matrix.set_value(1, 1, 2.0 / (top - bottom));
        matrix.set_value(2, 2, 2.0 / (near - far));

        matrix.set_value(0, 3, (left + right) / (left - right));
        matrix.set_value(1, 3, (bottom + top) / (bottom - top));
        matrix.set_value(2, 3, (far + near) / (far - near));

        matrix
    }

    pub fn perspective(fov: f32, width: u32, height: u32, near: f32, far: f32) -> Self {
        let aspect_ratio = (width / height) as f32;
        let x_scale = 1.0 / ((fov / 2.0).to_radians().tan());
        let y_scale = x_scale * aspect_ratio;
        let frustum_length = far - near;

        let mut matrix = Self::identity();

        matrix.set_value(0, 0, x_scale);
        matrix.set_value(1, 1, y_scale);
        matrix.set_value(2, 2, -((near + far) / frustum_length));
        matrix.set_value(3, 2, -1.0);
        matrix.set_value(2, 3, -((2.0 * near * far) / frustum_length));

        matrix
    }

    pub fn transformation(translation: &Vector3, rotation: &Vector3, scale: &Vector3) -> Self {
        let matrix = Self::identity();

        let matrix = matrix.translated(translation);
        let matrix = matrix.rotated_xyz(rotation.into());
        let matrix = matrix.scaled(scale);

        matrix
    }

    pub fn translated(&self, translation: &Vector3) -> Self {
        let mut matrix = self.clone();

        matrix.set_value(0, 3, matrix.get_value(0, 3) + translation.x);
        matrix.set_value(1, 3, matrix.get_value(1, 3) + translation.x);
        matrix.set_value(2, 3, matrix.get_value(2, 3) + translation.x);

        matrix
    }

    pub fn rotated_x(&self, angle: f32) -> Self {
        self.rotated(angle, &Vector3::new(1.0, 0.0, 0.0))
    }

    pub fn rotated_y(&self, angle: f32) -> Self {
        self.rotated(angle, &Vector3::new(0.0, 1.0, 0.0))
    }

    pub fn rotated_z(&self, angle: f32) -> Self {
        self.rotated(angle, &Vector3::new(0.0, 0.0, 1.0))
    }

    pub fn rotated_xyz(&self, angles: (f32, f32, f32)) -> Self {
        let matrix = self.rotated_x(angles.0);
        let matrix = matrix.rotated_y(angles.1);
        let matrix = matrix.rotated_z(angles.2);

        matrix
    }

    pub fn rotated(&self, angle: f32, axis: &Vector3) -> Self {
        let mut matrix = self.clone();

        let r = angle.to_radians();
        let c = r.cos();
        let s = r.sin();
        let omc = 1.0 - c;

        let (x, y, z) = axis.into();

        matrix.set_value(0, 0, x * omc + c);
        matrix.set_value(1, 0, y * x * omc + z * s);
        matrix.set_value(2, 0, x * z * omc - y * s);

        matrix.set_value(0, 1, x * y * omc - z * s);
        matrix.set_value(1, 1, y * omc + c);
        matrix.set_value(2, 1, y * z * omc + x * s);

        matrix.set_value(0, 2, x * z * omc + y * s);
        matrix.set_value(1, 2, y * z * omc - x * s);
        matrix.set_value(2, 2, z * omc + c);

        matrix
    }

    pub fn scaled(&self, scale: &Vector3) -> Self {
        let mut matrix = self.clone();

        matrix.set_value(0, 0, matrix.get_value(0, 0) * scale.x);
        matrix.set_value(1, 1, matrix.get_value(1, 1) * scale.y);
        matrix.set_value(2, 2, matrix.get_value(2, 2) * scale.z);

        matrix
    }

    #[inline]
    pub fn set_value(&mut self, i: usize, j: usize, value: f32) {
        debug_assert!(i < 4 && j < 4);
        self.0[i + j * 4] = value;
    }

    #[inline]
    pub fn get_value(&self, i: usize, j: usize) -> f32 {
        debug_assert!(i < 4 && j < 4);
        self.0[i + j * 4]
    }
}

use std::fmt;

impl fmt::Display for Matrix4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut row_dominant = [0.0; 4 * 4];
        for j in 0..4 {
            for i in 0..4 {
                row_dominant[j + i * 4] = self.get_value(i, j);
            }
        }

        write!(
            f,
            "{:?}\n{:?}\n{:?}\n{:?}",
            &row_dominant[0..4],
            &row_dominant[4..8],
            &row_dominant[8..12],
            &row_dominant[12..16]
        )
    }
}

use std::ops;

impl ops::Neg for Matrix4 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut array = [0.0; 4 * 4];
        for i in 0..self.0.len() {
            array[i] = -self.0[i];
        }
        Self(array)
    }
}

impl ops::Add for Matrix4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut matrix = self.clone();
        matrix += rhs;

        matrix
    }
}

impl ops::AddAssign for Matrix4 {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..self.0.len() {
            self.0[i] = self.0[i] + rhs.0[i];
        }
    }
}

impl ops::Sub for Matrix4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut matrix = self.clone();
        matrix -= rhs;

        matrix
    }
}

impl ops::SubAssign for Matrix4 {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..self.0.len() {
            self.0[i] = self.0[i] - rhs.0[i];
        }
    }
}

impl ops::Mul for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut matrix = self.clone();
        matrix *= rhs;

        matrix
    }
}

impl ops::MulAssign for Matrix4 {
    fn mul_assign(&mut self, rhs: Self) {
        let mut matrix = Self::zero();
        for j in 0..4 {
            for i in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self.get_value(i, k) * rhs.get_value(k, j);
                }
                matrix.set_value(i, j, sum);
            }
        }
        *self = matrix;
    }
}

impl ops::Mul<f32> for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut matrix = self.clone();
        matrix *= rhs;

        matrix
    }
}

impl ops::MulAssign<f32> for Matrix4 {
    fn mul_assign(&mut self, rhs: f32) {
        for i in 0..self.0.len() {
            self.0[i] = self.0[i] * rhs;
        }
    }
}

impl ops::Div<f32> for Matrix4 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        let mut matrix = self.clone();
        matrix /= rhs;

        matrix
    }
}

impl ops::DivAssign<f32> for Matrix4 {
    fn div_assign(&mut self, rhs: f32) {
        for i in 0..self.0.len() {
            self.0[i] = self.0[i] / rhs;
        }
    }
}

impl ops::Mul<Vector4> for Matrix4 {
    type Output = Vector4;

    fn mul(self, rhs: Vector4) -> Self::Output {
        // Switch to row-dominance for multiplication.
        let mut values = [0.0; 4 * 4];
        for j in 0..4 {
            for i in 0..4 {
                values[j + i * 4] = self.get_value(i, j);
            }
        }

        Vector4::new(
            values[0] * rhs.x + values[1] * rhs.y + values[2] * rhs.z + values[3] * rhs.w,
            values[4] * rhs.x + values[5] * rhs.y + values[6] * rhs.z + values[7] * rhs.w,
            values[8] * rhs.x + values[9] * rhs.y + values[10] * rhs.z + values[11] * rhs.w,
            values[12] * rhs.x + values[13] * rhs.y + values[14] * rhs.z + values[15] * rhs.w,
        )
    }
}
