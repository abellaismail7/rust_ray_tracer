use crate::utils::vec3::{Float, Vec3};

#[derive(Debug)]
pub struct Ray<'a> {
    pub org: &'a Vec3,
    pub dir: &'a Vec3,
}

impl<'a> Ray<'a> {
    pub fn new(org: &'a Vec3, dir: &'a Vec3) -> Self {
        Self { org, dir }
    }

    pub fn position(&self, t: Float) -> Vec3 {
        self.org + &(self.dir * t)
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
        let ray = Ray::new(&org, &dir);

        assert_eq!(ray.org, &org);
        assert_eq!(ray.dir, &dir);
    }

    #[test]
    fn test_position() {
        let org = Vec3::new(2.0, 3.0, 4.0);
        let dir = Vec3::new(1.0, 0.0, 0.0);
        let ray = Ray::new(&org, &dir);

        assert_eq!(ray.position(0.0), Vec3::new(2.0, 3.0, 4.0));
        assert_eq!(ray.position(1.0), Vec3::new(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), Vec3::new(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), Vec3::new(4.5, 3.0, 4.0));
    }
}
