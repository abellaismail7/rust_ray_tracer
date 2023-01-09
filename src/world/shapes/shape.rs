use std::fmt::Debug;

use crate::utils::{
    ray::Ray, vec3::{Vec3, Float}, material::Material,
};

pub trait Shape: Debug {
    fn intersect<'a>(&'a self, oray: &Ray, xs: &mut Vec<(&'a dyn Shape, Float)>);
    fn normal_at(&self, hitp: &Vec3) -> Vec3;
    fn material(&self) -> &Material;
}
