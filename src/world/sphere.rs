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
        (hitp).norm()
    }
}
