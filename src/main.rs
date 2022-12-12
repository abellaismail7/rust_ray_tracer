use vec3::{Vec3, Float};

use crate::canvas::Canvas;

pub mod vec3;
pub mod canvas;

fn get_ray(c: &Canvas, x: usize, y: usize) ->  Vec3 {
    let ar = c.ar;
    let angle = c.angle;

    let x = (2.0 * (x as Float) / (c.width  as Float) - 1.0) * angle * ar ;
    let y = (2.0 * (y as Float) / (c.height as Float) - 1.0) * angle ;

    Vec3::new(x, y, -1.0)
}

fn main() {
    let mut canvas = Canvas::new(1000, 1000);
    let c1 = canvas.clone();
    let org = Vec3::new(0.0, 0.0, -0.8);
    let light = - &Vec3::new(-4.0, 0.0, -2.0).norm();
    canvas.for_each(|pixel, x, y|{
        let dir = get_ray(&c1, x, y);
        //let dir = dir.norm();
        let mut color = Vec3::new(1.0, 0.0 , 1.0);

        let a: Float = dir.dot(&dir);
        let b: Float = org.dot(&dir) * 2.0;
        let c: Float = org.dot(&org) - (0.25);

        let d: Float = b * b - 4.0 * a * c;
        if d < 0.0
        {
            color.set_scalar(0.0, 0.0, 0.0);
            color.apply(pixel);
            return;
        }
        let t: Float = -b - d*d / 2.0 * a;
        let hitp = &(&org + &dir) * t;
        let norm = hitp.norm();
        let f = norm.dot(&light);
        color = &color * f;
        color.apply(pixel);
    });

    canvas.export_ppm("file.ppm").ok();
}
