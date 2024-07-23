use crate::{
    utils::{
        material::{IMaterial, Material},
        matrix::Mat,
        ray::Ray,
        vec3::{Float, Vec3, EPSILON},
    },
    world::transform::Transformable,
};

use super::shape::Shape;

#[derive(Debug)]
pub struct Plane {
    pub m: Material,
    pub t: Mat,
    pub inverse: Mat,
}

impl Shape for Plane {
    fn intersect(&self, oray: &Ray) -> Option<(Float, Float)> {
        let ray = oray.transform(&self.inverse);
        if ray.dir.y.abs() < EPSILON {
            return None;
        }
        let t = -ray.org.y / ray.dir.y;
        if t < EPSILON {
            return None;
        }
        Some((t, f32::INFINITY))
    }

    fn normal_at(&self, _hitp: &Vec3) -> Vec3 {
        Vec3::new(0.0, 1.0, 0.0)
    }

    fn get_material(&self) -> &Material {
        &self.m
    }
}

impl Transformable for Plane {
    #[inline]
    fn apply_transform(&mut self, transform: &Mat) {
        self.t = &self.t * transform;
        self.inverse = self.t.inverse();
    }
}

impl IMaterial for Plane {
    #[inline]
    fn get_material(&mut self) -> &mut Material {
        &mut self.m
    }
}

impl Plane {
    pub fn set_transform(&mut self, m: Mat) {
        self.inverse = m.inverse();
        self.t = m;
    }
}

impl Default for Plane {
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
