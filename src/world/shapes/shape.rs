use std::fmt::Debug;

use crate::utils::{
    ray::Ray, vec3::{Vec3, Float}, material::Material,
};

pub trait Shape: Debug {
    fn intersect(&mut self, oray: &Ray);
    fn normal_at(&self, hitp: &Vec3) -> Vec3;
    fn material(&self) -> &Material;
    fn get_intersections(&self) -> &[Float];
    fn intersected(&self) -> bool;
}
