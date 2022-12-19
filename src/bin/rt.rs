use std::f32::consts::PI;

use minirt::{
    scene::canvas::Canvas,
    utils::{
        material::Material,
        matrix::Mat,
        ray::Ray,
        vec3::Vec3,
    },
    world::{
        camera::Camera,
        light::Light,
        sphere::Sphere,
        w::{Comp, World},
    },
};

fn is_shadow(w: &World, ray: &Ray, comp: &Comp) -> bool {
    w.intersect(ray, Vec::new())
        .iter()
        .filter(|i| !std::ptr::eq(i.sp, comp.intersection.sp))
        .any(|i| i.t < 0.0)
}

fn shade_hit(w: &World, c: &Comp, l: &Light) -> Vec3 {
    let m = &c.intersection.sp.m;
    let color = &m.color * &l.intensity;

    let ambient = &color * m.ambient;
    let mut specular = Vec3::from_float(0.0);
    let mut diff = Vec3::from_float(0.0);

    let light_dir = (&l.position - &c.hitp).norm();
    let light_dot = light_dir.dot(&c.normalv);

    let intersected_with_light = light_dot >= 0.0;
    let ray = Ray::new(c.hitp.clone(), (-&light_dir).norm());
    if intersected_with_light && !is_shadow(w, &ray, c) {
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

fn trace(world: &World, ray: &Ray, depth: usize) -> Vec3 {
    let bg = Vec3::zero();
    if depth > 10 {
        return bg;
    }
    let container = Vec::with_capacity(world.spheres.len());
    let intersections = world.intersect(ray, container);
    if let Some(nearest) = intersections.first() {
        let hitp = ray.position(nearest.t);
        let norm = nearest.sp.normal_at(&hitp);
        let c = Comp {
            intersection: nearest,
            reflectv: -&ray.dir.reflect(&norm).norm(),
            hitp,
            normalv: norm,
            eyev: -&ray.dir,
            inside: false,
        };
        let mut color = Vec3::from_float(0.0);
        if nearest.sp.m.reflective > 0.0 {
            color = trace(world, &Ray::new(c.hitp.clone(), c.reflectv.clone() ), depth + 1) * nearest.sp.m.reflective;
        }
        for light in world.lights.iter() {
            color = color + shade_hit(world, &c, light);
        }
        return color;
    }
    bg
}

fn main() {
    let camera = Camera::new(
        1000,
        1000,
        PI * 0.33,
        Mat::view_transformation(
            &Vec3::new(0.0, 1.5, -5.0),
            &Vec3::new(0.0, 1.0, 0.0),
            &Vec3::new(0.0, 1.0, 0.0),
        ),
    );

    let mut canvas = Canvas::new(camera.width, camera.height);

    let lights = vec![
        Light::new(Vec3::new(-10.0, 10.0, -10.0), Vec3::from_float(1.0)),
        //Light::new(Vec3::new(-10.5, 1.0, -10.75), Vec3::from_float(1.0)),
    ];

    let m = Material {
        color: Vec3::new(1.0, 0.9, 0.9),
        specular: 0.0,
        ..Material::default()
    };

    let spheres = vec![
        //Sphere::new(m.clone(), Mat::identity(4).scaling(10.0, 0.01, 10.0)),
        //Sphere::new(
        //    Material {
        //        color: Vec3::new(1.0, 0.9, 0.9),
        //        ..m
        //    },
        //    Mat::identity(4)
        //        .translation(0.0, 0.0, 5.0)
        //        .rotation_y(-PI / 4.0)
        //        .rotation_x(PI / 2.0)
        //        .scaling(10.0, 0.01, 10.0),
        //),
        //Sphere::new(
        //    Material {
        //        color: Vec3::new(1.0, 0.9, 0.9),
        //        ..m
        //    },
        //    Mat::identity(4)
        //        .translation(0.0, 0.0, 5.0)
        //        .rotation_y(PI / 4.0)
        //        .rotation_x(PI / 2.0)
        //        .scaling(10.0, 0.01, 10.0),
        //),
        Sphere::new(
            Material {
                color: Vec3::new(0.0, 1.0, 1.0),
                diffuse: 1.0,
                reflective: 0.5,
                ..Material::default()
            },
            Mat::identity(4)
                .translation(-0.5, 1.0, 0.5)
                .scaling(1.0, 1.0, 1.0),
        ),
        Sphere::new(
            Material {
                color: Vec3::new(1.0, 0.2, 1.0),
                diffuse: 1.0,
                ..Material::default()
            },
            Mat::identity(4)
                .translation(1.5, 0.5, -0.5)
                .scaling(0.5, 0.2, 0.5),
        ),
        Sphere::new(
            Material {
                color: Vec3::new(1.0, 1.0, 0.0),
                diffuse: 1.0,
                ..Material::default()
            },
            Mat::identity(4)
                .translation(-1.5, 1.0, -0.5)
                .scaling(0.33, 0.33, 0.33),
        ),
    ];

    let w = World::new(camera, lights, spheres);
    canvas.for_each(|pixel, x, y| {
        let ray = w.camera.get_ray(x, y);
        let color = trace(&w, &ray, 0);
        print!("\r{} pixel", 1000 * y + x);
        color.apply(pixel)
    });
    canvas.export_ppm("file.ppm").ok();
}
