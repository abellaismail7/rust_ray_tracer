use crate::utils::{
    material::Material,
    matrix::Mat,
    ray::Ray,
    vec3::{Float, Vec3},
};

use super::{
    camera::{self, Camera},
    light::Light,
    sphere::Sphere,
};

#[derive(Debug)]
pub struct World {
    pub camera: Camera,
    pub lights: Vec<Light>,
    pub spheres: Vec<Sphere>,
}

#[derive(Debug)]
pub struct Intersection<'a> {
    pub sp: &'a Sphere,
    pub t0: Float,
    pub t1: Float,
}

#[derive(Debug)]
pub struct Comp<'a> {
    pub intersection: &'a Intersection<'a>,
    pub hitp: Vec3,
    pub normalv: Vec3,
    pub eyev: Vec3,
    pub inside: bool,
}

impl World {
    pub fn new(camera: Camera, lights: Vec<Light>, spheres: Vec<Sphere>) -> Self {
        Self {
            camera,
            lights,
            spheres,
        }
    }

    pub fn default() -> Self {
        let camera = Camera::new(1000, 1000, 45.0, Mat::identity(4));
        let lights = vec![Light::new(
            Vec3::new(-10.0, 10.0, -10.0),
            Vec3::new(1.0, 1.0, 1.0),
        )];
        let spheres = vec![
            Sphere::new(
                Material {
                    color: Vec3::new(0.8, 1.0, 0.6),
                    diffuse: 0.7,
                    specular: 0.2,
                    ..Material::default()
                },
                Mat::identity(4),
            ),
            Sphere::new(
                Material {
                    ..Material::default()
                },
                Mat::identity(4).scaling(0.5, 0.5, 0.5),
            ),
        ];

        World {
            camera,
            lights,
            spheres,
        }
    }

    pub fn intersect<'a, 'b>(
        &'a self,
        ray: &Ray,
        mut vec: Vec<Intersection<'b>>,
    ) -> Vec<Intersection<'b>>
    where
        'a: 'b,
    {
        let iter = self
            .spheres
            .iter()
            .filter_map(|s| {
                let p = s.intersect(ray)?;
                Some((s, p.0, p.1))
            })
            .map(|(s, t0, t1)| {
                if t1 < t0 {
                    // I don't think that's imposible
                    Intersection { sp: s, t1, t0 }
                } else {
                    Intersection { sp: s, t0, t1 }
                }
            });
        vec.extend(iter);
        vec.sort_by(|a, b| a.t0.partial_cmp(&b.t0).unwrap());
        vec
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

        assert_eq!(is.len(), 2);
        assert_eq!(is[0].t0, 4.0);
        assert_eq!(is[0].t1, 6.0);
        assert_eq!(is[1].t0, 4.5);
        assert_eq!(is[1].t1, 5.5);
    }
}
