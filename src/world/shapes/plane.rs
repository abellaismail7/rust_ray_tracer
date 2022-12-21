/* use crate::{utils::{
    material::{IMaterial, Material},
    matrix::Mat,
    ray::Ray,
    vec3::{Vec3, EPSILON},
}, world::{transform::Transformable, w::Intersection}};

use super::shape::Shape;

#[derive(Debug)]
pub struct Plane {
    pub m: Material,
    pub t: Mat,
    pub inverse: Mat,
}

impl Shape for Plane {

    fn normal_at(&self, _hitp: &Vec3) -> Vec3 {
        let obj_norm = Vec3::new(0.0, 1.0, 0.0);
        let wrl_norm = &self.inverse.transpose() * &obj_norm;
        (wrl_norm).norm()
    }

    fn material(&self) -> &Material {
        &self.m
    }

    fn intersect<'a: 'b, 'b>(_w: &'a World, &self, oray: &Ray, vec: &'b mut Vec<Intersection<'b>>) -> bool {
        let ray = oray.transform(&self.inverse);
        if ray.dir.y.abs() < EPSILON
        {
            return false;
        }
        vec.push(Intersection{sp: self, t: -ray.org.y / ray.dir.y});
        true
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
*/
