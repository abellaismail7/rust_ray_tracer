use crate::vec3::{Float, Vec3};
use std::f32;

#[derive(Debug)]
pub struct Camera {
    pub org: Vec3,
    pub forword: Vec3,
    pub right: Vec3,
    pub up: Vec3,
    pub w: Float,
    pub h: Float,
    pub x_step: Float,
    pub y_step: Float,
}

impl Camera {
    pub fn new(
        org: Vec3,
        forword: Vec3,
        up: Vec3,
        fov: Float,
        width: usize,
        height: usize,
    ) -> Self {
        let right = forword.cross(&up);
        let ar = width as Float / (height as Float);
        let h = (f32::consts::PI * 0.5 * fov / 180.).tan();
        let w = h * ar;
        Self {
            org,
            forword,
            right,
            up,
            w,
            h,
            x_step: 2.0 / (width as Float),
            y_step: 2.0 / (height as Float),
        }
    }
}
