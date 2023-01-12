use std::fmt::Debug;

use crate::utils::{
    material::Material,
    ray::Ray,
    vec3::{Float, Vec3}, matrix::Mat,
};

pub trait Shape: Debug {
    fn intersect<'a>(&'a self, oray: &Ray, xs: &mut Vec<(&'a dyn Shape, Float)>);
    fn normal_at(&self, hitp: &Vec3) -> Vec3;
    fn material(&self) -> &Material;
    fn inverse(&self) -> &Mat;

    fn world_point(&self, point: &Vec3) -> Vec3 {
        let obj_norm = self.inverse() * point;
        obj_norm
    }
}
