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

    pub fn transform(&self, m: &Mat) -> Self {
        let mm = &m; // this a tar9i3 til I could work arround it
        Self {
            org: m * &self.org,
            dir: mm * &self.dir,
        }
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
    fn test_transform_1() {
        let r1 = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(0.0, 1.0, 0.0));
        let r2 = Ray::new(Vec3::new(4.0, 6.0, 8.0), Vec3::new(0.0, 1.0, 0.0));
        let t = Mat::identity(4).translation(3.0, 4.0, 5.0);
        let r1 = r1.transform(&t);

        assert_eq!(r1, r2);
    }

    #[test]
    fn test_transform_2() {
        let r1 = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(0.0, 1.0, 0.0));
        let r2 = Ray::new(Vec3::new(2.0, 6.0, 12.0), Vec3::new(0.0, 3.0, 0.0));
        let t = Mat::identity(4).scaling(2.0, 3.0, 4.0);
        let r1 = r1.transform(&t);

        assert_eq!(r1, r2);
    }
}
