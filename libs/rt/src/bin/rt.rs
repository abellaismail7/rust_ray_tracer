use rt::{
    rt::RayTracer,
    scene::canvas::Canvas,
    utils::{material::IMaterial, matrix::Mat, vec3::Vec3},
    world::{
        camera::Camera, light::Light, shapes::sphere::Sphere, transform::Transformable, w::World,
    },
};
use std::f32::consts::PI;

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
        Light::new(Vec3::new(-10.0, 10.0, -10.0), Vec3::new(1.0, 0.5, 1.0)),
        Light::new(Vec3::new(-10.5, 1.0, -10.75), Vec3::from_float(1.0)),
    ];

    let spheres = vec![
        Sphere::default()
            .color(0.0, 1.0, 1.0)
            .diffuse(0.7)
            .reflective(0.5)
            .specular(1.0)
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

    let rt = RayTracer::new(World::new(camera, lights, spheres));
    canvas.for_each(|pixel, x, y| {
        let ray = rt.world().camera.get_ray(x, y);
        let color = rt.trace(&ray, 0);
        color.apply(pixel)
    });

    canvas.export_ppm("file.ppm").ok();
}
