use super::{camera::Camera, light::Light, sphere::Sphere};

#[derive(Debug)]
pub struct World {
    pub camera: Camera,
    pub lights: Vec<Light>,
    pub spheres: Vec<Sphere>,
}

impl World {
    pub fn new(camera: Camera, lights: Vec<Light>, spheres: Vec<Sphere>) -> Self {
        Self {
            camera,
            lights,
            spheres,
        }
    }
}
