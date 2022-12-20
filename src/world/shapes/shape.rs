use crate::utils::{
    ray::Ray,
    vec3::{Float, Vec3},
};

pub trait Shape {
    fn intersect(&self, oray: &Ray) -> Option<(Float, Float)>;
    fn normal_at(&self, hitp: &Vec3) -> Vec3;
}
