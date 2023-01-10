use crate::world::shapes::shape::Shape;

use super::{
    ray::Ray,
    vec3::{Float, Vec3, EPSILON},
};

#[derive(Debug)]
pub struct Comp<'a> {
    pub cur_shape: &'a dyn Shape,
    pub hitp: Vec3,
    pub normalv: Vec3,
    pub reflectv: Vec3,
    pub eyev: Vec3,
    pub inside: bool,
}

impl<'a> Comp<'a> {
    pub fn prepare_comp(ray: &Ray, sh: &'a dyn Shape, t: Float) -> Comp<'a> {
        let hitp = ray.position(t);
        let normalv = sh.normal_at(&hitp);
        Self {
            cur_shape: sh,
            reflectv: ray.dir.reflect(&normalv),
            normalv,
            hitp,
            eyev: -&ray.dir,
            inside: false,
        }
    }
}
