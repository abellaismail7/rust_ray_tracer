use std::ops::{Add, Div, Mul, Neg, Sub};

pub type Float = f32;
pub const EPSILON: Float = 0.0001;

#[derive(Debug, Clone)]
pub struct Vec3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

impl Vec3 {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn from_float(f: Float) -> Self {
        Self { x: f, y: f, z: f }
    }

    pub fn cross(&self, other: &Vec3) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn dot(&self, rhs: &Vec3) -> Float {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn dotxz(&self, rhs: &Vec3) -> Float {
        self.x * rhs.x + self.z * rhs.z
    }

    pub fn mag(&self) -> Float {
        self.dot(self).sqrt()
    }

    pub fn norm(&self) -> Vec3 {
        self / self.mag()
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        self - normal * self.dot(normal) * 2.0
    }

    pub fn apply(&self, pixel: &mut [u8]) {
        pixel[0] = (self.x.clamp(0.0, 1.0) * 255.0) as u8;
        pixel[1] = (self.y.clamp(0.0, 1.0) * 255.0) as u8;
        pixel[2] = (self.z.clamp(0.0, 1.0) * 255.0) as u8;
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

    pub fn to_color(&self) -> u32 {
        let r = (self.x.clamp(0.0, 1.0) * 255.0) as u32;
        let g = (self.y.clamp(0.0, 1.0) * 255.0) as u32;
        let b = (self.z.clamp(0.0, 1.0) * 255.0) as u32;
        r << 16 | g << 8 | b
    }
}

macro_rules! vec_ops {
    ($imp:ident, $method:ident, $t:ty) => {
        impl $imp for $t {
            type Output = Vec3;

            #[inline]
            fn $method(self, rhs: $t) -> Self::Output {
                Vec3 {
                    x: self.x.$method(rhs.x),
                    y: self.y.$method(rhs.y),
                    z: self.z.$method(rhs.z),
                }
            }
        }

        impl $imp for &$t {
            type Output = Vec3;

            #[inline]
            fn $method(self, rhs: &$t) -> Self::Output {
                Vec3 {
                    x: self.x.$method(rhs.x),
                    y: self.y.$method(rhs.y),
                    z: self.z.$method(rhs.z),
                }
            }
        }

        impl $imp<&$t> for $t {
            type Output = Vec3;

            #[inline]
            fn $method(self, rhs: &$t) -> Self::Output {
                Vec3 {
                    x: self.x.$method(rhs.x),
                    y: self.y.$method(rhs.y),
                    z: self.z.$method(rhs.z),
                }
            }
        }

        impl $imp<$t> for &$t {
            type Output = Vec3;

            #[inline]
            fn $method(self, rhs: $t) -> Self::Output {
                Vec3 {
                    x: self.x.$method(rhs.x),
                    y: self.y.$method(rhs.y),
                    z: self.z.$method(rhs.z),
                }
            }
        }
    };

    ($imp:ident, $method:ident, $t:ty, $rhs:ident) => {
        impl $imp<$rhs> for $t {
            type Output = Vec3;

            fn $method(self, rhs: $rhs) -> Self::Output {
                Vec3 {
                    x: self.x.$method(rhs),
                    y: self.y.$method(rhs),
                    z: self.z.$method(rhs),
                }
            }
        }

        impl $imp<$rhs> for &$t {
            type Output = Vec3;

            fn $method(self, rhs: $rhs) -> Self::Output {
                Vec3 {
                    x: self.x.$method(rhs),
                    y: self.y.$method(rhs),
                    z: self.z.$method(rhs),
                }
            }
        }
    };
}

vec_ops! {Add, add, Vec3 }
vec_ops! {Add, add, Vec3, Float }

vec_ops! {Sub, sub, Vec3 }
vec_ops! {Sub, sub, Vec3, Float }

vec_ops! {Div, div, Vec3 }
vec_ops! {Div, div, Vec3, Float }

vec_ops! {Mul, mul, Vec3 }
vec_ops! {Mul, mul, Vec3, Float }

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        let x_diff = (self.x - other.x).abs();
        let y_diff = (self.y - other.y).abs();
        let z_diff = (self.z - other.z).abs();

        x_diff < EPSILON && y_diff < EPSILON && z_diff < EPSILON
    }
}

impl Neg for &Vec3 {
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
    fn test_dot_vec3_vec3_1() {
        let v1 = Vec3::new(6.0, 7.0, 4.0);
        let v2 = Vec3::new(2.0, 3.0, 2.0);

        let res = v1.dot(&v2);
        assert_eq!(res, 12.0 + 21.0 + 8.0);
    }

    #[test]
    fn test_dot_vec3_vec3_2() {
        let v1 = Vec3::from_float(1.0);
        let v2 = Vec3::new(-2.0, 2.0, 0.0);
        let v2_norm = Vec3::new(-2.0, 2.0, 0.0).norm();

        let res = v1.dot(&v2);
        let res_n = v1.dot(&v2_norm);
        assert_eq!(res, 0.0);
        assert_eq!(res, res_n);
    }

    #[test]
    fn test_reflect_1() {
        let v1 = Vec3::new(1.0, -1.0, 0.0);
        let normal = &Vec3::new(0.0, 1.0, 0.0).norm();

        let r = v1.reflect(normal);
        assert_eq!(r, Vec3::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn test_reflect_2() {
        let twosqrt2 = 2.0_f32.powf(0.5) / 2.0;
        let v1 = Vec3::new(0.0, -1.0, 0.0);
        let normal = &Vec3::new(twosqrt2, twosqrt2, 0.0).norm();

        let r = v1.reflect(normal);
        assert_eq!(r, Vec3::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_cross_vec3_vec3_1() {
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);

        let res = v1.cross(&v2);
        assert_eq!(res, Vec3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_cross_vec3_vec3_2() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);

        let res = v1.cross(&v2);

        assert_eq!(res, Vec3::new(-1.0, 2.0, -1.0));
    }

    #[test]
    fn test_cross_vec3_vec3_3() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);

        let res = v2.cross(&v1);

        assert_eq!(res, Vec3::new(1.0, -2.0, 1.0));
    }

    #[test]
    fn test_from_float() {
        assert_eq!(Vec3::new(2.0, 2.0, 2.0), Vec3::from_float(2.0))
    }
}
