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

fn lighting(m: &Material, lights: &[Light], eye_vn: &Vec3, normal_v: &Vec3, hitp: &Vec3) -> Vec3 {

    let l = &lights[0];
    let color = &m.color * &l.intensity;

    let ambient =  &color * m.ambient;
    let mut specular = Vec3::from_float(0.0);
    let mut diff = Vec3::from_float(0.0);

    let light_dir = (&l.position - hitp).norm();
    let light_dot = light_dir.dot(normal_v);

    if light_dot >= 0.0 {
        diff = &(&color * m.diffuse) * normal_v.dot(&light_dir);

        let reflect = (-&light_dir).reflect(normal_v).norm();
        let reflect_dot = reflect.dot(eye_vn);

        if reflect_dot > 0.0 {
            let factor =  reflect_dot.powf(m.shininess);
            specular = &(&l.intensity * m.specular) * factor;
        }
    }
    &(&ambient + &diff) + &specular
}

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
            lighting(&s.m, lights, &(-ray.dir).norm(), &norm, &hitp)
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
                color: Vec3::new(1.0, 0.2, 1.0),
                ..Material::default()
            },
            1.0,
        ),
    ];
    let camera = Camera::new(
        Vec3::new(0.0, 0.0, -5.0),
        Vec3::new(0.0, 0.0, 1.0).norm(),
        Vec3::new(0.0, 1.0, 0.0),
        45.0,
        canvas.width,
        canvas.height,
    );
    let lights = vec![Light::new(
        Vec3::new(-10.0, 10.0, -10.0),
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
