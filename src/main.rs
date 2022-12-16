use scene::canvas::Canvas;
use utils::{
    material::Material,
    ray::Ray,
    vec3::{Float, Vec3},
};
use world::{camera::Camera, light::Light, sphere::Sphere};

pub mod scene;
pub mod utils;
pub mod world;

fn trace(ray: &Ray, spheres: &[Sphere], lights: &[Light]) -> Vec3 {
    let mut sphere: Option<&Sphere> = None;
    let mut t = Float::INFINITY;
    for s in spheres.iter() {
        if let Some((t0, _t1)) = s.intersect(ray.org, ray.dir) {
            if t0 < t {
                sphere = Some(s);
                t = t0;
            }
        };
    }
    let bg = Vec3::new(0.0, 0.0, 0.0);
    match sphere {
        Some(s) => {
            let hitp = ray.position(t);
            let norm = s.normal_at(&hitp);
            let f = norm.dot(&lights[0].position);
            &s.m.color * f
        }
        None => bg,
    }
}

fn main() {
    let mut canvas = Canvas::new(1000, 1000);
    let spheres = vec![
        Sphere::new(
            Vec3::new(1.0, 1.5, -1.0),
            Material {
                color: Vec3::new(1.0, 0.2, 1.0),
                ..Material::default()
            },
            0.5,
        ),
        Sphere::new(
            Vec3::new(0.0, 0.0, 0.0),
            Material {
                color: Vec3::new(0.0, 1.0, 1.0),
                ..Material::default()
            },
            1.0,
        ),
    ];
    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 3.0),
        Vec3::new(0.0, 0.0, 1.0).norm(),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        canvas.width,
        canvas.height,
    );
    let lights = vec![Light::new(
        Vec3::new(-2.0, -2.0, -2.0),
        Vec3::from_float(1.0),
    )];
    canvas.for_each(|pixel, x, y| {
        let dir = camera.get_ray(x, y);
        let ray = Ray::new(&camera.org, &dir);
        let color = trace(&ray, &spheres, &lights);
        color.apply(pixel)
    });
    canvas.export_ppm("file.ppm").ok();
}
