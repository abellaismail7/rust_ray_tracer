use std::f32::consts::PI;
use minirt::{
    scene::canvas::Canvas,
    utils::{matrix::Mat, vec3::Vec3, material::IMaterial},
    world::{camera::Camera, light::Light,  w::World, shapes::{sphere::Sphere, plane::Plane, shape::Shape}, transform::Transformable},
};

fn main() {
    let camera = Camera::new(
        1000,
        1000,
        PI * 0.33,
        Mat::view_transformation(
            &Vec3::new(0.0, 1.5, -5.0),
            &Vec3::new(0.0, 1.0, 0.0 ),
            &Vec3::new(0.0, 1.0, 0.0 ),
        ),
    );

    let mut canvas = Canvas::new(camera.width, camera.height);

    let lights = vec![
        Light::new(Vec3::new(-10.0, 10.0, -10.0), Vec3::new(0.5, 0.5, 0.5)),
        //Light::new(Vec3::new(-10.5, 1.0, -10.75), Vec3::from_float(1.0)),
    ];

    let spheres: Vec<Box<dyn Shape>> = vec![
        Box::new(Sphere::default()
            .color(0.0, 1.0, 1.0)
            .reflective(0.5)
            .translation(-0.5, 1.0, 0.5)
            .scaling(1.0, 1.0, 1.0)),
        Box::new(Sphere::default()
            .color(1.0, 0.2, 1.0)
            .translation(0.5, -0.0, -0.5)
            .scaling(0.5, 0.2, 0.5)),
        Box::new(Sphere::default()
            .color(1.0, 1.0, 0.0)
            .reflective(0.2)
            .specular(1.0)
            .translation(-1.5, 1.0, -0.5)
            .scaling(0.33, 0.33, 0.33)),
        //Box::new(Plane::default().reflective(0.2)),
    ];

    let w = World::new(camera, lights, spheres);
    canvas.for_each(|pixel, x, y| {
        let ray = w.camera.get_ray(x, y);
        let color = minirt::trace(&w, &ray, 0);
        //print!("\r{} pixel", 1000 * y + x);
        color.apply(pixel)
    });
    canvas.export_ppm("file.ppm").ok();
}
