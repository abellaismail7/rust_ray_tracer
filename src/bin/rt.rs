use std::f32::consts::PI;

use minirt::{
    scene::canvas::Canvas,
    utils::{comp::Comp, material::IMaterial, matrix::Mat, ray::Ray, vec3::Vec3},
    world::{camera::Camera, light::Light, sphere::Sphere, transform::Transformable, w::World},
};

fn is_shadow(w: &World, ray: &Ray, comp: &Comp) -> bool {
    w.intersect(ray, Vec::new())
        .iter()
        .filter(|i| !std::ptr::eq(i.sp, comp.intersection.sp))
        .any(|i| i.t < 0.0)
}

fn shade_hit(w: &World, c: &Comp, light: &Light) -> Vec3 {
    let mut specular = Vec3::zero();
    let mut diff = Vec3::zero();

    let m = &c.intersection.sp.m;
    let color = &m.color * &light.intensity;
    let ray = light.ray_at(&c.hitp);
    let light_dot = (-&ray.dir).dot(&c.normalv);

    let intersected_with_light = light_dot >= 0.0;
    if intersected_with_light && !is_shadow(w, &ray, c) {
        diff = &color * m.diffuse * light_dot;

        let reflect = ray.dir.reflect(&c.normalv);
        let reflect_dot = reflect.dot(&c.eyev);

        if reflect_dot > 0.0 {
            let factor = reflect_dot.powf(m.shininess);
            specular = &light.intensity * m.specular * factor;
        }
    }
    (&color * m.ambient) + diff + specular
}

fn reflected_color(world: &World, comp: &Comp, depth: usize) -> Vec3 {
    let nearest = comp.intersection;
    if nearest.sp.m.reflective > 0.0 && depth < 10 {
        trace(
            world,
            &Ray::new(comp.hitp.clone(), comp.reflectv.clone()),
            depth + 1,
        ) * nearest.sp.m.reflective
    } else {
        Vec3::zero()
    }
}

fn trace(world: &World, ray: &Ray, depth: usize) -> Vec3 {
    let bg = Vec3::zero();
    if depth > 10 {
        return bg;
    }
    let container = Vec::with_capacity(world.spheres.len());
    let intersections = world.intersect(ray, container);
    if let Some(nearest) = intersections.first() {
        let comps = Comp::prepare_comp(ray, nearest);
        let mut surface = reflected_color(world, &comps, depth);
        for light in world.lights.iter() {
            surface = surface + shade_hit(world, &comps, light);
        }
        return surface;
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
        Light::new(Vec3::new(-10.0, 10.0, -10.0), Vec3::new(1.0, 0.0, 1.0)),
        //Light::new(Vec3::new(-10.5, 1.0, -10.75), Vec3::from_float(1.0)),
    ];

    let spheres = vec![
        Sphere::default()
            .color(0.0, 1.0, 1.0)
            .diffuse(0.7)
            .reflective(0.5)
            .translation(-0.5, 1.0, 0.5)
            .scaling(1.0, 1.0, 1.0),
        Sphere::default()
            .color(1.0, 0.2, 1.0)
            .diffuse(0.7)
            .translation(0.5, -0.0, -0.5)
            .scaling(0.5, 0.2, 0.5),
        Sphere::default()
            .color(1.0, 1.0, 0.0)
            .diffuse(0.7)
            .specular(1.0)
            .translation(-1.5, 1.0, -0.5)
            .scaling(0.33, 0.33, 0.33),
    ];

    let w = World::new(camera, lights, spheres);
    canvas.for_each(|pixel, x, y| {
        let ray = w.camera.get_ray(x, y);
        let color = trace(&w, &ray, 0);
        //print!("\r{} pixel", 1000 * y + x);
        color.apply(pixel)
    });
    canvas.export_ppm("file.ppm").ok();
}
