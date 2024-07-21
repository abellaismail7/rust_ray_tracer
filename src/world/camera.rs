use crate::utils::{
    matrix::Mat,
    ray::Ray,
    vec3::{Float, Vec3},
};

#[derive(Debug)]
pub struct Camera {
    pub width: u32,
    pub height: u32,
    pub x_step: Float,
    pub y_step: Float,
    ar: Float,
    angle: Float,
    inverse: Mat,
    fov: f32,
}

impl Camera {
    pub fn new(width: u32, height: u32, fov: Float, t: Mat) -> Self {
        let ar = width as Float / (height as Float);
        let angle = (fov * 0.5).tan();
        Self {
            width,
            height,
            ar,
            angle,
            x_step: (2.0 / (width as Float)),
            y_step: (2.0 / (height as Float)),
            inverse: t.inverse(),
            fov,
        }
    }

    pub fn update_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.ar = self.width as Float / (self.height as Float);
        self.angle = (self.fov * 0.5).tan();
        self.x_step = 2.0 / (width as Float);
        self.y_step = 2.0 / (height as Float);
    }

    pub fn update(&mut self, fov: f32, t: Mat) {
        self.ar = self.width as Float / (self.height as Float);
        self.angle = (fov * 0.5).tan();
        self.inverse = t.inverse();
    }

    pub fn get_ray(&self, x: u32, y: u32) -> Ray {
        let x = -(self.x_step * (x as Float) - 1.0) * self.ar * self.angle;
        let y = -(self.y_step * (y as Float) - 1.0) * self.angle;

        let p = &self.inverse * &Vec3::new(x, y, -1.0);
        let o = &self.inverse * &Vec3::from_float(0.0);
        let dir = (p - &o).norm();
        Ray::new(o, dir)
    }
}
