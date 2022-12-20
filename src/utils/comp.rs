use crate::world::w::Intersection;

use super::{ray::Ray, vec3::Vec3};

#[derive(Debug)]
pub struct Comp<'a> {
    pub intersection: &'a Intersection<'a>,
    pub hitp: Vec3,
    pub normalv: Vec3,
    pub reflectv: Vec3,
    pub eyev: Vec3,
    pub inside: bool,
}

impl<'a> Comp<'a> {
    pub fn prepare_comp(ray: &Ray, nearest: &'a Intersection) -> Comp<'a> {
        let hitp = ray.position(nearest.t);
        let norm = nearest.sp.normal_at(&hitp);
        Self {
            intersection: nearest,
            reflectv: -&ray.dir.reflect(&norm),
            normalv: nearest.sp.normal_at(&hitp),
            hitp,
            eyev: -&ray.dir,
            inside: false,
        }
    }
}
