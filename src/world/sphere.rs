use crate::utils::{
    material::Material,
    vec3::{Float, Vec3}, ray::Ray,
};

#[derive(Debug)]
pub struct Sphere {
    center: Vec3,
    raduis: Float,
    pub m: Material,
}

impl Sphere {
    pub fn new(center: Vec3, m: Material, raduis: Float) -> Self {
        Self { center, raduis, m }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<(Float, Float)> {
        let oc = ray.org - &self.center;
        let a: Float = ray.dir.dot(ray.dir);
        let b2: Float = oc.dot(ray.dir);
        let c: Float = oc.dot(&oc) - (self.raduis * self.raduis);

        let d: Float = b2 * b2 - a * c;
        if d < 0.0 {
            return None;
        }
        let t0: Float = -b2 - d.sqrt() / a;
        let t1: Float = -b2 + d.sqrt() / a;
        Some((t0, t1))
    }

    pub fn normal_at(&self, hitp: &Vec3) -> Vec3 {
        (hitp - &self.center).norm()
    }
}
