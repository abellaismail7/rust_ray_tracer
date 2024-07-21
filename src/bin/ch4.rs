use minirt::{
    scene::canvas::Canvas,
    utils::{matrix::Mat, vec3::Vec3},
    world::transform::Transformable,
};
use std::f32::consts::PI;

fn main() {
    let mut canvas = Canvas::new(300, 300);
    let mut p = Vec3::new(0.0, 0.5, 0.0);
    let m = Mat::identity(4).rotation_z(PI / 6.0);
    let white = Vec3::from_float(1.0);
    for _ in 0..12 {
        let x = ((p.x + 0.5) * 200.0) as u32 + 50;
        let y = ((p.y + 0.5) * 200.0) as u32 + 50;
        canvas.write_at(x, y, &white);
        p = &m * &p;
    }
    canvas.export_ppm("file.ppm").ok();
}
