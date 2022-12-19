use std::f32::consts::PI;

use minirt::{
    scene::canvas::Canvas,
    utils::{material::Material, matrix::Mat, ray::Ray, vec3::Vec3},
    world::{
        camera::Camera,
        light::Light,
        sphere::Sphere,
        w::{Comp, World},
    },
};

fn shade_hit(c: &Comp, l: &Light) -> Vec3 {
    let m = &c.intersection.sp.m;
    let color = &m.color * &l.intensity;

    let ambient = &color * m.ambient;
    let mut specular = Vec3::from_float(0.0);
    let mut diff = Vec3::from_float(0.0);

    let light_dir = (&l.position - &c.hitp).norm();
    let light_dot = light_dir.dot(&c.normalv);

    if light_dot >= 0.0 {
        diff = &color * m.diffuse * light_dot;

        let reflect = (-&light_dir).reflect(&c.normalv);
        let reflect_dot = reflect.dot(&c.eyev);

        if reflect_dot > 0.0 {
            let factor = reflect_dot.powf(m.shininess);
            specular = &l.intensity * m.specular * factor;
        }
    }
    ambient + diff + specular
}

fn trace(w: &World, ray: &Ray) -> Vec3 {
    let bg = Vec3::new(0.0, 0.0, 0.0);
    let intersections = Vec::with_capacity(w.spheres.len());
    let intersections = w.intersect(ray, intersections);
    if let Some(i) = intersections.first() {
        let hitp = ray.position(i.t0);
        let norm = i.sp.normal_at(&hitp);
        let c = Comp {
            intersection: i,
            hitp,
            normalv: norm,
            eyev: -&ray.dir,
            inside: false,
        };
        let mut color = Vec3::from_float(0.0);
        for l in w.lights.iter() {
            color = color + shade_hit(&c, l);
        }
        return color;
    }
    bg
}

fn main() {
    let camera = Camera::new(
        1000,
        1000,
        90.0,
        Mat::view_transformation(
            &Vec3::new(0.0, 1.5, -5.0),
            &Vec3::new(0.0, 1.0, 0.0),
            &Vec3::new(0.0, 1.0, 0.0),
        ),
    );

    let mut canvas = Canvas::new(camera.width, camera.height);

    let lights = vec![
        Light::new(Vec3::new(-10.0, 10.0, -10.0), Vec3::from_float(1.0)),
        // Light::new(Vec3::new(-0.0, -3.0, -0.0), Vec3::from_float(1.0)),
    ];

    let m = Material {
        color: Vec3::new(1.0, 0.9, 0.9),
        specular: 0.0,
        ..Material::default()
    };

    let spheres = vec![
        // Sphere::new(
        //     m.clone(),
        //     Mat::identity(4).scaling(10.0, 0.01, 10.0),
        // ),
        // Sphere::new(
        //     Material{
        //         color: Vec3::new(0.0, 0.0, 1.0),
        //         ..m
        //     },
        //     Mat::identity(4)
        //         .translation(0.0, 0.0, 0.5)
        //         .rotation_y(-PI / 4.0)
        //         .rotation_x( PI / 2.0)
        //         .scaling(10.0, 0.01, 14.0),
        // ),
        // Sphere::new(
        //     Material{
        //         color: Vec3::new(1.0, 0.0, 0.0),
        //         ..m
        //     },
        //     Mat::identity(4)
        //         .translation(0.0, 0.0, 5.0)
        //         .rotation_y( PI / 4.0)
        //         .rotation_x( PI / 2.0)
        //         .scaling(20.0, 0.01, 17.0),
        // ),
        Sphere::new(
            Material {
                color: Vec3::new(0.0, 1.0, 1.0),
                ..Material::default()
            },
            Mat::identity(4)
                .translation(-0.5, 1.0, 0.5)
                .scaling(1.5, 1.5, 0.5),
        ),
        Sphere::new(
            Material {
                color: Vec3::new(1.0, 0.2, 1.0),
                ..Material::default()
            },
            Mat::identity(4).translation(1.5, 0.5, -0.5),
        ),
        Sphere::new(
            Material {
                color: Vec3::new(1.0, 1.0, 0.0),
                ..Material::default()
            },
            Mat::identity(4)
                .translation(-1.5, 0.33, -0.75)
                .scaling(0.5, 0.5, 0.5),
        ),
    ];

    let w = World::new(camera, lights, spheres);
    canvas.for_each(|pixel, x, y| {
        let ray = w.camera.get_ray(x, y);
        let color = trace(&w, &ray);
        print!("\r{} pixel", 1000 * y + x);
        color.apply(pixel)
    });
    canvas.export_ppm("file.ppm").ok();
}
