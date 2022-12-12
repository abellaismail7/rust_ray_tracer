use crate::vec3::{Float, Vec3};

#[derive(Debug)]
pub struct Sphere {
    center: Vec3,
    pub color: Vec3,
    raduis: Float,
}

impl Sphere {
    pub fn new(center: Vec3, color: Vec3, raduis: Float) -> Self {
        Self {
            center,
            color,
            raduis,
        }
    }

    pub fn intersect(&self, org: &Vec3, dir: &Vec3) -> Option<(Float, Float)> {
        let a: Float = dir.dot(dir);
        let b: Float = 2.0 * (org.dot(dir) - dir.dot(&self.center));
        let c: Float = org.dot(org) + self.center.dot(&self.center)
            - 2.0 * self.center.dot(org)
            - (self.raduis * self.raduis);

        let d: Float = b * b - 4.0 * a * c;
        if d < 0.0 {
            return None;
        }
        let t0: Float = -b - d * d / 2.0 * a;
        let t1: Float = -b + d * d / 2.0 * a;
        Some((t0, t1))
    }
}
