use super::matrix::Mat;
use crate::utils::vec3::{Float, Vec3};

#[derive(Debug, PartialEq)]
pub struct Ray {
    pub org: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(org: Vec3, dir: Vec3) -> Self {
        Self { org, dir }
    }

    pub fn position(&self, t: Float) -> Vec3 {
        &self.org + &self.dir * t
    }

    pub fn transform(&self, m: &Mat) -> Ray {
        Ray::new(m * &self.org, self.dir.clone())
    }

    pub fn transform_ref(&mut self, m: &Mat) {
        self.org = m * &self.org;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::vec3::Vec3;

    #[test]
    fn test_new() {
        let org = Vec3::new(2.0, 3.0, 4.0);
        let dir = Vec3::new(1.0, 0.0, 0.0);
        let ray = Ray::new(org.clone(), dir.clone());

        assert_eq!(ray.org, org);
        assert_eq!(ray.dir, dir);
    }

    #[test]
    fn test_position() {
        let org = Vec3::new(2.0, 3.0, 4.0);
        let dir = Vec3::new(1.0, 0.0, 0.0);
        let ray = Ray::new(org, dir);

        assert_eq!(ray.position(0.0), Vec3::new(2.0, 3.0, 4.0));
        assert_eq!(ray.position(1.0), Vec3::new(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), Vec3::new(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), Vec3::new(4.5, 3.0, 4.0));
    }

    #[test]
    fn test_transform() {
        let mut r1 = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::from_float(1.0));
        let r2 = Ray::new(Vec3::new(4.0, 6.0, 8.0), r1.dir.clone());
        let t = Mat::identity(4).translation(3.0, 4.0, 5.0);
        r1.transform_ref(&t);

        assert_eq!(r1, r2);
    }
}
