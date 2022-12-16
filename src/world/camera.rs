use crate::utils::vec3::{Float, Vec3};

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
        let h = (fov.to_radians() * 0.5).tan();
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

    pub fn get_ray(&self, x: usize, y: usize) -> Vec3 {
        let x = -(self.x_step * (x as Float) - 1.0);
        let y = -(self.y_step * (y as Float) - 1.0);
        let dir = &self.forword + &(&(&self.up * self.h) * y);
        &dir + &(&(&self.right * self.w) * x)
    }
}
