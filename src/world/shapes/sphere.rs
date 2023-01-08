use crate::{utils::{
    material::{IMaterial, Material},
    matrix::Mat,
    ray::Ray,
    vec3::{Float, Vec3}, intersection_holder::IntersectionHolder,
}, world::transform::Transformable};

use super::shape::Shape;

#[derive(Debug)]
pub struct Sphere {
    pub m: Material,
    pub t: Mat,
    pub inverse: Mat,
    pub intersections: [Float; 2],
    pub intersected: bool,
}

impl Shape for Sphere {

    fn intersect<'a>(&'a self, oray: &Ray, xs: & mut IntersectionHolder<(&'a dyn Shape, f32)>) {
        let ray = oray.transform(&self.inverse);
        let a: Float = ray.dir.dot(&ray.dir);
        let b2: Float = ray.dir.dot(&ray.org);
        let c: Float = ray.org.dot(&ray.org) - 1.0;

        let d: Float = b2.powf(2.0) - (a * c);
        if d < 0.0 {
            return ;
        }
        let d_sqrt = d.sqrt();
        
        xs.push((self, (-b2 - d_sqrt) / a));
        xs.push((self, (-b2 + d_sqrt) / a));
    }

    fn normal_at(&self, hitp: &Vec3) -> Vec3 {
        let obj_norm = (&self.inverse * hitp).norm();
        let wrl_norm = &self.inverse.transpose() * &obj_norm;
        (wrl_norm).norm()
    }

    fn material(&self) -> &Material {
        &self.m
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

    fn new() -> Box<Self> {
        let t = Mat::identity(4);
        let inverse = t.inverse();
        Box::new(Self {
            m: Material::default(),
            t,
            inverse,
            intersections: [0.0; 2],
            intersected: false,
        })
    }

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
            intersections: [0.0; 2],
            intersected: false,
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
