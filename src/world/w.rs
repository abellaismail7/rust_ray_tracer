use crate::utils::{
    material::IMaterial,
    matrix::Mat,
    ray::Ray,
    vec3::{Float, Vec3},
};

use super::{camera::Camera, light::Light, shapes::{shape::Shape, sphere::Sphere}, transform::Transformable};

type Intersections<'a> = Vec<(&'a Box<dyn Shape>, Float)>;

#[derive(Debug)]
pub struct World {
    pub camera: Camera,
    pub lights: Vec<Light>,
    pub shapes: Vec<Box<dyn Shape>>,
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
        &'a mut self,
        ray: &Ray,
    ) -> Intersections<'a> {
        self.shapes.iter_mut().for_each(|sh| sh.intersect(ray));

        let mut xs: Intersections = self.shapes.iter()
        .filter(|sh| sh.intersected())
        .flat_map(|sh| {
            sh.get_intersections().iter().map(|f| (&*sh, *f))
        })
        .collect();

        xs.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
        xs
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

        World::new(camera, lights, shapes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect() {
        let w = World::default();
        let r = Ray::new(Vec3::new(0.0, 0.0, -5.0), Vec3::new(0.0, 0.0, 1.0));
        let xs = w.intersect(&r);



        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].1, 4.0);
        assert_eq!(xs[1].1, 4.5);
        assert_eq!(xs[2].1, 5.5);
        assert_eq!(xs[3].1, 6.0);
    }
}
