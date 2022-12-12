use sphere::Sphere;
use std::f32;
use vec3::{Float, Vec3};

use crate::canvas::Canvas;

pub mod canvas;
pub mod sphere;
pub mod vec3;

fn get_ray(c: &Canvas, x: usize, y: usize) -> Vec3 {
    let x = 2.0 * (x as Float) / (c.width as Float) - 1.0;
    let y = 2.0 * (y as Float) / (c.height as Float) - 1.0;
    let up = &Vec3::new(0.0, 1.0, 0.0);
    let forword = &Vec3::new(0.0, 0.0, 1.0).norm();
    let right = &forword.cross(up);
    let h = (f32::consts::PI * 0.5 * 70.0 / 180.).tan();
    let w = h * c.ar;
    let dir = forword + &(&(up * h) * (y as Float));
    &dir + &(&(right * w) * (x as Float))
}

fn main() {
    let mut canvas = Canvas::new(1000, 1000);
    let c1 = canvas.clone();
    let org = Vec3::new(0.0, 0.0, 1.0);
    let light = Vec3::new(-1.0, -1.0, -1.0).norm();
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 1.0), 0.5);
    let bg = Vec3::new(0.0, 0.0, 0.0);
    canvas.for_each(|pixel, x, y| {
        let dir = get_ray(&c1, x, y);
        match sphere.intersect(&org, &dir) {
            Some((t0, _t1)) => {
                let hitp = &(&org + &dir) * t0;
                let norm = hitp.norm();
                let f = norm.dot(&light);
                let color = &sphere.color * f;
                color.apply(pixel);
            }
            None => bg.apply(pixel),
        };
    });

    canvas.export_ppm("file.ppm").ok();
}
