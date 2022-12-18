use crate::utils::{
    ray::Ray,
    vec3::{Float, Vec3},
};

use super::{camera::Camera, light::Light, sphere::Sphere};

#[derive(Debug)]
pub struct World {
    pub camera: Camera,
    pub lights: Vec<Light>,
    pub spheres: Vec<Sphere>,
}

#[derive(Debug)]
pub struct Intersection<'a> {
    pub sp: &'a Sphere,
    pub t0: Float,
    pub t1: Float,
}

#[derive(Debug)]
pub struct Comp<'a> {
    pub intersection: &'a Intersection<'a>,
    pub hitp: Vec3,
    pub normalv: Vec3,
    pub eyev: Vec3,
    pub inside: bool,
}

impl World {
    pub fn new(camera: Camera, lights: Vec<Light>, spheres: Vec<Sphere>) -> Self {
        Self {
            camera,
            lights,
            spheres,
        }
    }

    pub fn intersect<'a, 'b>(
        &'a self,
        ray: &Ray,
        mut vec: Vec<Intersection<'b>>,
    ) -> Vec<Intersection<'b>>
    where
        'a: 'b,
    {
        let iter = self
            .spheres
            .iter()
            .filter_map(|s| {
                let p = s.intersect(ray)?;
                Some((s, p.0, p.1))
            })
            .map(|(s, t0, t1)| {
                if t1 < t0 {
                    // I don't think that's imposible
                    Intersection { sp: s, t1, t0 }
                } else {
                    Intersection { sp: s, t0, t1 }
                }
            });
        vec.extend(iter);
        vec.sort_by(|a, b| a.t0.partial_cmp(&b.t0).unwrap());
        vec
    }
}
