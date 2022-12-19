use crate::utils::{
    matrix::Mat,
    ray::Ray,
    vec3::{Float, Vec3},
};

#[derive(Debug)]
pub struct Camera {
    pub width: usize,
    pub height: usize,
    pub x_step: Float,
    pub y_step: Float,
    ar: Float,
    angle: Float,
    inverse: Mat,
}

impl Camera {
    pub fn new(width: usize, height: usize, fov: Float, t: Mat) -> Self {
        let ar = width as Float / (height as Float);
        let angle = (fov.to_radians() * 0.5).tan();
        Self {
            width,
            height,
            ar,
            angle,
            x_step: (2.0 / (width as Float)),
            y_step: (2.0 / (height as Float)),
            inverse: t.inverse(),
        }
    }

    pub fn get_ray(&self, x: usize, y: usize) -> Ray {
        let x = -(self.x_step * (x as Float) - 1.0) * self.ar * self.angle;
        let y = -(self.y_step * (y as Float) - 1.0) * self.angle;

        let p = &self.inverse * &Vec3::new(x, y, -1.0);
        let o = &self.inverse * &Vec3::from_float(0.0);
        let dir = (p - &o).norm();
        Ray::new(o, dir)
    }
}
