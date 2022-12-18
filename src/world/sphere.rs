use crate::utils::{
    material::Material,
    matrix::Mat,
    ray::Ray,
    vec3::{Float, Vec3},
};

#[derive(Debug)]
pub struct Sphere {
    pub m: Material,
    pub t: Mat,
    pub inverse: Mat,
}

impl Sphere {
    pub fn new(m: Material, t: Mat) -> Self {
        let inverse = t.inverse();
        Self { m, t, inverse }
    }

    pub fn intersect(&self, oray: &Ray) -> Option<(Float, Float)> {
        let ray = oray.transform(&self.inverse);
        let a: Float = ray.dir.dot(&ray.dir);
        let b2: Float = ray.org.dot(&ray.dir);
        let c: Float = ray.org.dot(&ray.org) - 1.0;

        let d: Float = b2 * b2 - a * c;
        if d < 0.0 {
            return None;
        }
        let t0: Float = -b2 - d.sqrt() / a;
        let t1: Float = -b2 + d.sqrt() / a;
        Some((t0, t1))
    }

    pub fn set_transform(&mut self, m: Mat) {
        self.inverse = m.inverse();
        self.t = m;
    }

    pub fn normal_at(&self, hitp: &Vec3) -> Vec3 {
        let obj_norm = (&self.inverse * hitp).norm();
        let wrl_norm= &self.inverse.transpose() * &obj_norm;
        (wrl_norm).norm()
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use super::*;

    #[test]
    fn test_normal_at_1() {
        let f = std::f32::consts::FRAC_1_SQRT_2;
        let sp = Sphere::new(Material::default(),
            Mat::identity(4).translation(-0.0, 1.0, -0.0));
        let v = sp.normal_at(&Vec3::new(0.0, 1.70711, -f));
        assert_eq!(v, Vec3::new(0.0, f, -f));
    }

    #[test]
    fn test_normal_at_2() {
        let f = 2.0f32.sqrt()/ 2.0;
        let sp = Sphere::new(Material::default(),
            Mat::identity(4).scaling(1.0, 0.5, 1.0).rotation_z(PI/5.0));
        let v = sp.normal_at(&Vec3::new(0.0, f, -f));
        assert_eq!(v, Vec3::new(0.0, 0.97014, -0.24254));
    }
}
