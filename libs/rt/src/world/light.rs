use crate::utils::{ray::Ray, vec3::Vec3};

#[derive(Debug)]
pub struct Light {
    pub position: Vec3,
    pub intensity: Vec3,
}

impl Light {
    pub fn new(position: Vec3, intensity: Vec3) -> Self {
        Self {
            position,
            intensity,
        }
    }

    pub fn ray_at(&self, hitp: &Vec3) -> Ray {
        let light_dir = (hitp - &self.position).norm();
        Ray::new(hitp.clone(), light_dir)
    }
}
