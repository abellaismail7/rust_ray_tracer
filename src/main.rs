use scene::canvas::Canvas;
use utils::vec3::{Float, Vec3};
use world::{camera::Camera, sphere::Sphere};


pub mod world;
pub mod scene;
pub mod utils;

fn get_ray(c: &Camera, x: usize, y: usize) -> Vec3 {
    let x = c.x_step * (x as Float) - 1.0;
    let y = c.y_step * (y as Float) - 1.0;
    let dir = &c.forword + &(&(&c.up * c.h) * y);
    &dir + &(&(&c.right * c.w) * x)
}

fn get_camera(c: &Canvas) -> Camera {
    let org = Vec3::new(0.0, 0.0, 1.0);
    let forword = Vec3::new(0.0, 0.0, 1.0).norm();
    let up = Vec3::new(0.0, 1.0, 0.0);
    Camera::new(org, forword, up, 45.0, c.width, c.height)
}

fn main() {
    let spheres = vec![
        Sphere::new(Vec3::new(0.0, -100.5, 4.0), Vec3::new(0.5, 1.0, 0.5), 100.0),
        Sphere::new(Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 1.0), 0.5),
    ];
    let light = -&Vec3::new(-0.0, 0.0, 3.0).norm();
    let bg = Vec3::new(0.0, 0.0, 0.0);
    {
        let mut canvas = Canvas::new(400, 225);
        let camera = get_camera(&canvas);
        canvas.for_each(|pixel, x, y| {
            let dir = get_ray(&camera, x, y);
            let mut sphere: Option<&Sphere> = None;
            let mut t = f32::INFINITY;
            for s in spheres.iter() {
                if let Some((t0, _t1)) = s.intersect(&camera.org, &dir) {
                    if t0 < t {
                        sphere = Some(s);
                        t = t0;
                    }
                };
            }
            match sphere {
                Some(s) => {
                    let hitp = &(&camera.org + &dir) * t;
                    let norm = hitp.norm();
                    let f = norm.dot(&light);
                    let color = &s.color * f;
                    color.apply(pixel);
                }
                None => bg.apply(pixel),
            }
        });
        canvas.export_ppm("file.ppm").ok();
    }
}
