use crate::utils::{
    material::IMaterial,
    matrix::Mat,
    ray::Ray,
    vec3::{Float, Vec3},
};

use super::{camera::Camera, light::Light, shapes::{shape::Shape, sphere::Sphere}, transform::Transformable};

#[derive(Debug)]
pub struct World {
    pub camera: Camera,
    pub lights: Vec<Light>,
    pub shapes: Vec<Box<dyn Shape>>,
}

#[derive(Debug)]
pub struct Intersection<'a> {
    pub sp: &'a dyn Shape,
    pub t: Float,
}

impl World {
    pub fn new(camera: Camera, lights: Vec<Light>, shapes: Vec<Box<dyn Shape>>) -> Self {
        Self {
            camera,
            lights,
            shapes,
        }
    }

    pub fn intersect<'a>(
        &'a self,
        ray: &Ray,
        mut vec: &'a mut  Vec<Intersection<'a>>,
    )
    {
        for sh in self.shapes.iter() {
            sh.intersect(ray, &mut vec);
        }
        vec.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    }
}

impl Default for World {
    fn default() -> Self {
        let camera = Camera::new(1000, 1000, 45.0, Mat::identity(4));
        let lights = vec![Light::new(
            Vec3::new(-10.0, 10.0, -10.0),
            Vec3::new(1.0, 1.0, 1.0),
        )];
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Sphere::default()
                .color(0.8, 1.0, 0.6)
                .diffuse(0.7)
                .specular(0.5)),
            Box::new(Sphere::default().scaling(0.5, 0.5, 0.5)),
        ];

        World {
            camera,
            lights,
            shapes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect() {
        let w = World::default();
        let r = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
        let is = w.intersect(&r, Vec::new());

        assert_eq!(is.len(), 4);
        assert_eq!(is[0].t, 4.0);
        assert_eq!(is[1].t, 4.5);
        assert_eq!(is[2].t, 5.5);
        assert_eq!(is[3].t, 6.0);
    }
}
