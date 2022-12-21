use std::fmt::Debug;

use crate::{utils::{
    ray::Ray,
    vec3::Vec3, material::Material,
}, world::w::Intersection};

pub trait Shape: Debug {
    fn intersect<'a: 'b, 'b>(&'a self, oray: &Ray, vec: &'b mut Vec<Intersection<'b>>) -> bool;
    fn normal_at(&self, hitp: &Vec3) -> Vec3;
    fn material(&self) -> &Material;
}
