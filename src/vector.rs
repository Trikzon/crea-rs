pub trait SizedVector {
    fn size() -> usize;
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<(f32, f32, f32)> for Vec3f {
    fn from(data: (f32, f32, f32)) -> Self {
        Vec3f {
            x: data.0,
            y: data.1,
            z: data.2,
        }
    }
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3f { x, y, z }
    }
}

impl SizedVector for Vec3f {
    fn size() -> usize {
        3
    }
}

impl std::ops::Add for Vec3f {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::AddAssign for Vec3f {
    fn add_assign(&mut self, other: Self) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
    }
}

impl std::ops::Sub for Vec3f {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::SubAssign for Vec3f {
    fn sub_assign(&mut self, other: Self) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
        self.z = self.z - other.z;
    }
}

impl std::ops::Mul for Vec3f {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl std::ops::MulAssign for Vec3f {
    fn mul_assign(&mut self, other: Self) {
        self.x = self.x * other.x;
        self.y = self.y * other.y;
        self.z = self.z * other.z;
    }
}

impl std::ops::Div for Vec3f {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Vec3f {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl std::ops::DivAssign for Vec3f {
    fn div_assign(&mut self, other: Self) {
        self.x = self.x / other.x;
        self.y = self.y / other.y;
        self.z = self.z / other.z;
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Vec4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl From<(f32, f32, f32, f32)> for Vec4f {
    fn from(data: (f32, f32, f32, f32)) -> Self {
        Vec4f {
            x: data.0,
            y: data.1,
            z: data.2,
            w: data.3,
        }
    }
}

impl SizedVector for Vec4f {
    fn size() -> usize {
        4
    }
}

impl std::ops::Add for Vec4f {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl std::ops::AddAssign for Vec4f {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
}

impl std::ops::Sub for Vec4f {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl std::ops::SubAssign for Vec4f {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
    }
}

impl std::ops::Mul for Vec4f {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w,
        }
    }
}

impl std::ops::MulAssign for Vec4f {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
        self.w *= other.w;
    }
}

impl std::ops::Div for Vec4f {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w,
        }
    }
}

impl std::ops::DivAssign for Vec4f {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
        self.w /= other.w;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec3f_eq() {
        let vec1 = Vec3f::new(1.0, 0.0, -1.0);
        let vec2 = Vec3f::new(1.0, 0.0, -1.0);
        assert_eq!(vec1, vec2);
    }

    #[test]
    fn vec3f_ne() {
        let vec1 = Vec3f::new(1.0, 0.0, -1.0);
        let vec2 = Vec3f::new(0.0, -1.0, 1.0);
        assert_ne!(vec1, vec2);
    }

    #[test]
    fn vec3f_add() {
        let vec1 = Vec3f::new(1.0, 0.0, -1.0);
        let vec2 = Vec3f::new(1.0, 0.0, -1.0);
        assert_eq!(vec1 + vec2, Vec3f::new(2.0, 0.0, -2.0));
    }

    #[test]
    fn vec3f_add_assign() {
        let mut vec1 = Vec3f::new(1.0, 0.0, -1.0);
        vec1 += Vec3f::new(1.0, 0.0, -1.0);
        assert_eq!(vec1, Vec3f::new(2.0, 0.0, -2.0));
    }

    #[test]
    fn vec3f_sub() {
        let vec1 = Vec3f::new(1.0, 0.0, -1.0);
        let vec2 = Vec3f::new(1.0, 0.0, -1.0);
        assert_eq!(vec1 - vec2, Vec3f::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn vec3f_sub_assign() {
        let mut vec1 = Vec3f::new(1.0, 0.0, -1.0);
        vec1 -= Vec3f::new(1.0, 0.0, -1.0);
        assert_eq!(vec1, Vec3f::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn vec3f_mul() {
        let vec1 = Vec3f::new(1.0, 0.0, -1.0);
        let vec2 = Vec3f::new(1.0, 0.0, -1.0);
        assert_eq!(vec1 * vec2, Vec3f::new(1.0, 0.0, 1.0));
    }

    #[test]
    fn vec3f_mul_assign() {
        let mut vec1 = Vec3f::new(1.0, 0.0, -1.0);
        vec1 *= Vec3f::new(1.0, 0.0, -1.0);
        assert_eq!(vec1, Vec3f::new(1.0, 0.0, 1.0));
    }

    #[test]
    fn vec3f_div() {
        let vec1 = Vec3f::new(1.0, 0.0, -1.0);
        let vec2 = Vec3f::new(1.0, 1.0, -1.0);
        assert_eq!(vec1 / vec2, Vec3f::new(1.0, 0.0, 1.0));
    }

    #[test]
    fn vec3f_div_assign() {
        let mut vec1 = Vec3f::new(1.0, 0.0, -1.0);
        vec1 /= Vec3f::new(1.0, 1.0, -1.0);
        assert_eq!(vec1, Vec3f::new(1.0, 0.0, 1.0));
    }
}
