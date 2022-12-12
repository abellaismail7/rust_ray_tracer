use std::ops::{Add, Div, Mul, Neg, Sub};

pub type Float = f32;

#[derive(Debug, Clone, PartialEq)]
pub struct Vec3 {
    x: Float,
    y: Float,
    z: Float,
}

impl Vec3 {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Self { x, y, z }
    }

    pub fn from_float(f: Float) -> Self {
        Self { x: f, y: f, z: f }
    }

    pub fn dot(&self, rhs: &Vec3) -> Float {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn mag(&self) -> Float {
        self.dot(self).sqrt()
    }

    pub fn norm(&self) -> Vec3 {
        self / self.mag()
    }

    pub fn apply(&self, pixel: &mut [u8]) {
        pixel[0] = Self::clamp(self.x);
        pixel[1] = Self::clamp(self.y);
        pixel[2] = Self::clamp(self.z);
    }

    pub fn set(&mut self, other: &Vec3) {
        self.x = other.x;
        self.y = other.y;
        self.z = other.z;
    }

    pub fn set_scalar(&mut self, x: Float, y: Float, z: Float) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    fn clamp(f: Float) -> u8 {
        if f > 1.0 {
            return 255;
        } else if f < 0.0 {
            return 0;
        }
        (f * 255.0) as u8
    }
}

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<Float> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Float) -> Self::Output {
        Vec3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<Float> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Float) -> Self::Output {
        Vec3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl Mul for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<Float> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Float) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Div<Float> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Float) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Neg for &Vec3 {
    // add code here
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_vec3_vec3() {
        let v1 = Vec3::new(1.0, 2.0, 5.0);
        let v2 = Vec3::new(0.0, 3.0, 4.0);

        let v = &v1 + &v2;

        assert_eq!(v, Vec3::new(1.0, 5.0, 9.0));
    }

    #[test]
    fn test_add_vec3_float() {
        let v1 = Vec3::new(1.0, 1.0, 1.0);

        let v = &v1 + 1.0;

        assert_eq!(v, Vec3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn test_sub_vec3_float() {
        let v1 = Vec3::new(3.0, 3.0, 3.0);

        let v = &v1 - 1.0;

        assert_eq!(v, Vec3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn test_sub_vec3_vec3() {
        let v1 = Vec3::new(3.0, 3.0, 3.0);
        let v2 = Vec3::new(1.0, 1.0, 1.0);

        let v = &v1 - &v2;

        assert_eq!(v, Vec3::new(2.0, 2.0, 2.0));
    }

    #[test]
    fn test_mul_vec3_float() {
        let v1 = Vec3::new(3.0, 3.0, 3.0);

        let v = &v1 * 2.0;

        assert_eq!(v, Vec3::new(6.0, 6.0, 6.0));
    }

    #[test]
    fn test_mul_vec3_vec3() {
        let v1 = Vec3::new(3.0, 3.0, 3.0);
        let v2 = Vec3::new(2.0, 2.0, 2.0);

        let v = &v1 * &v2;
        assert_eq!(v, Vec3::new(6.0, 6.0, 6.0));
    }

    #[test]
    fn test_div_vec3_float() {
        let v1 = Vec3::new(6.0, 6.0, 6.0);

        let v = &v1 / 2.0;

        assert_eq!(v, Vec3::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn test_div_vec3_vec3() {
        let v1 = Vec3::new(6.0, 6.0, 6.0);
        let v2 = Vec3::new(2.0, 2.0, 2.0);

        let v = &v1 / &v2;
        assert_eq!(v, Vec3::new(3.0, 3.0, 3.0));
    }

    #[test]
    fn test_dot_vec3_vec3() {
        let v1 = Vec3::new(6.0, 7.0, 4.0);
        let v2 = Vec3::new(2.0, 3.0, 2.0);

        let res = v1.dot(&v2);
        assert_eq!(res, 12.0 + 21.0 + 8.0);
    }

    #[test]
    fn test_from_float() {
        assert_eq!(Vec3::new(2.0, 2.0, 2.0), Vec3::from_float(2.0))
    }
}
