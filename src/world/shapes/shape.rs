use std::fmt::Debug;

use crate::utils::{
    material::Material,
    ray::Ray,
    vec3::{Float, Vec3},
};

pub trait Shape: Debug {
    fn intersect<'a>(&'a self, oray: &Ray, xs: &mut Vec<(&'a dyn Shape, Float)>);
    fn normal_at(&self, hitp: &Vec3) -> Vec3;
    fn material(&self) -> &Material;
}
