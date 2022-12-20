use crate::utils::{
    material::{IMaterial, Material},
    matrix::Mat,
    ray::Ray,
    vec3::{Float, Vec3},
};

use super::{shape::Shape, transform::Transformable};

#[derive(Debug)]
pub struct Sphere {
    pub m: Material,
    pub t: Mat,
    pub inverse: Mat,
}

impl Shape for Sphere {
    fn intersect(&self, oray: &Ray) -> Option<(Float, Float)> {
        let ray = oray.transform(&self.inverse);
        let a: Float = ray.dir.dot(&ray.dir);
        let b2: Float = ray.dir.dot(&ray.org);
        let c: Float = ray.org.dot(&ray.org) - 1.0;

        let d: Float = b2.powf(2.0) - (a * c);
        if d < 0.0 {
            return None;
        }
        let d_sqrt = d.sqrt();
        let t0 = (-b2 - d_sqrt) / a;
        let t1 = (-b2 + d_sqrt) / a;
        Some((t0, t1))
    }

    fn normal_at(&self, hitp: &Vec3) -> Vec3 {
        let obj_norm = (&self.inverse * hitp).norm();
        let wrl_norm = &self.inverse.transpose() * &obj_norm;
        (wrl_norm).norm()
    }
}

impl Transformable for Sphere {
    #[inline]
    fn apply_transform(&mut self, transform: &Mat) {
        self.t = &self.t * transform;
        self.inverse = self.t.inverse();
    }
}

impl IMaterial for Sphere {
    #[inline]
    fn get_material(&mut self) -> &mut Material {
        &mut self.m
    }
}

impl Sphere {
    pub fn set_transform(&mut self, m: Mat) {
        self.inverse = m.inverse();
        self.t = m;
    }
}

impl Default for Sphere {
    fn default() -> Self {
        let t = Mat::identity(4);
        let inverse = t.inverse();
        Self {
            m: Material::default(),
            t,
            inverse,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use super::*;

    #[test]
    fn test_normal_at_1() {
        let f = std::f32::consts::FRAC_1_SQRT_2;
        let sp = Sphere::default().translation(-0.0, 1.0, -0.0);
        let v = sp.normal_at(&Vec3::new(0.0, 1.70711, -f));
        assert_eq!(v, Vec3::new(0.0, f, -f));
    }

    #[test]
    fn test_normal_at_2() {
        let f = 2.0f32.sqrt() / 2.0;
        let sp = Sphere::default()
            .scaling(1.0, 0.5, 1.0)
            .rotation_z(PI / 5.0);
        let v = sp.normal_at(&Vec3::new(0.0, f, -f));
        assert_eq!(v, Vec3::new(0.0, 0.97014, -0.24254));
    }
}
