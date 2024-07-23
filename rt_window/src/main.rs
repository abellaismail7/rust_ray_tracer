extern crate glutin_window;
extern crate graphics;
extern crate image as im;
extern crate opengl_graphics;
extern crate piston;

use opengl_graphics::OpenGL;
use piston::input::RenderEvent;
use piston::window::WindowSettings;
use piston_window::*;
use rt::{
    rt::RayTracer,
    utils::{material::IMaterial, matrix::Mat, vec3::Vec3},
    world::{
        camera::Camera,
        light::Light,
        shapes::{cylinder::Cylinder, plane::Plane, shape::Shape, sphere::Sphere},
        transform::Transformable,
        w::World,
    },
};
use std::f32::consts::PI;
//const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WIDTH: u32 = 600;

fn main() {
    let opengl = OpenGL::V3_2;
    let (width, height) = (WIDTH, WIDTH);
    let mut window: PistonWindow = WindowSettings::new("piston: paint", (width, height))
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();
    // Create a new game and run it.

    let mut canvas = im::ImageBuffer::new(WIDTH, WIDTH);
    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };
    let mut texture: G2dTexture =
        Texture::from_image(&mut texture_context, &canvas, &TextureSettings::new()).unwrap();
    let camera = Camera::new(
        WIDTH,
        WIDTH,
        PI * 0.33,
        Mat::view_transformation(
            &Vec3::new(0.0, 0.0, 5.0),
            &Vec3::new(0.0, 0.0, 0.0),
            &Vec3::new(0.0, 1.0, 0.0),
        ),
    );

    let lights = vec![
        Light::new(Vec3::new(0.0, 0.0, -5.0), Vec3::new(1.0, 0.5, 1.0)),
        //Light::new(Vec3::new(-10.5, 1.0, -10.75), Vec3::from_float(1.0)),
    ];

    let spheres: Vec<Box<dyn Shape>> = vec![
        Box::new(
            Sphere::default()
                .color(0.0, 1.0, 1.0)
                .diffuse(0.7)
                .reflective(0.5)
                .specular(1.0)
                .translation(0.0, 0.0, 0.0)
                .scaling(1.0, 1.0, 1.0),
        ),
        //Box::new(
        //    Sphere::default()
        //        .color(1.0, 0.2, 1.0)
        //        .diffuse(0.7)
        //        .translation(0.5, -0.0, -0.5)
        //        .scaling(0.5, 0.2, 0.5),
        //),
        //Box::new(
        //    Sphere::default()
        //        .color(1.0, 1.0, 0.0)
        //        .diffuse(0.7)
        //        .specular(1.0)
        //        .translation(-1.5, 1.0, -0.5)
        //        .scaling(0.33, 0.33, 0.33),
        //),
        Box::new(Plane::default().translation(-5.0, -5.0, -5.0)),
        Box::new(Cylinder::default().translation(-0.0, -0.0, -0.0)),
    ];

    let mut rt = RayTracer::new(World::new(camera, lights, spheres));

    while let Some(e) = window.next() {
        if e.render_args().is_some() {
            for x in 0..WIDTH {
                for y in 0..WIDTH {
                    let ray = rt.world().camera.get_ray(x, y);
                    let color = rt.trace(&ray, 0);
                    let mut pixel = [0_u8, 0, 0, 255];
                    color.apply(&mut pixel);
                    canvas.put_pixel(x, y, im::Rgba(pixel));
                }
            }
            texture.update(&mut texture_context, &canvas).unwrap();
            window.draw_2d(&e, |c, g, device| {
                // Update texture before rendering.
                texture_context.encoder.flush(device);

                clear([1.0; 4], g);
                image(&texture, c.transform, g);
            });
        }
        if let Some(key) = e.press_args() {
            if key == Button::Keyboard(Key::X) {
                rt.mut_world().camera.apply_rotation_x(0.1);
            }
            if key == Button::Keyboard(Key::Y) {
                rt.mut_world().camera.apply_rotation_y(0.1);
            }
            if key == Button::Keyboard(Key::Z) {
                rt.mut_world().camera.apply_rotation_z(0.1);
            }
            if key == Button::Keyboard(Key::Up) {
                rt.mut_world().camera.apply_translation(0.0, 0.1, 0.0);
            }
            if key == Button::Keyboard(Key::Down) {
                rt.mut_world().camera.apply_translation(0.0, -0.1, 0.0);
            }
            if key == Button::Keyboard(Key::Left) {
                rt.mut_world().camera.apply_translation(0.1, 0.0, 0.0);
            }
            if key == Button::Keyboard(Key::Right) {
                rt.mut_world().camera.apply_translation(-0.1, 0.0, 0.0);
            }
        }
    }
}

