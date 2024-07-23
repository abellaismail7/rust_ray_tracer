use crate::utils::{matrix::Mat, vec3::Float};

#[macro_export]
macro_rules! transformfun {
    ($func_name:ident; $func_name_apply:ident ($self:ident, $($param:ident : $type:ty),*) $block:block) => {
        fn $func_name(mut $self, $($param: $type),*) -> Self {
            $block
            $self
        }

        fn $func_name_apply(&mut $self, $($param: $type),*) {
            $block
        }
    };
}
pub trait Transformable: Sized {
    transformfun!(translation; apply_translation (self, x: Float, y: Float, z: Float) {
        let mut m = Mat::identity(4);
        m.tab[0][3] = x;
        m.tab[1][3] = y;
        m.tab[2][3] = z;
        self.apply_transform(&m);
    });

    fn scaling(mut self, x: Float, y: Float, z: Float) -> Self {
        let mut m = Mat::identity(4);
        m.tab[0][0] = x;
        m.tab[1][1] = y;
        m.tab[2][2] = z;
        self.apply_transform(&m);
        self
    }

    transformfun!(rotation_x; apply_rotation_x (self, r : Float) {
        let mut m = Mat::identity(4);
        m.tab[1][1] = r.cos();
        m.tab[1][2] = -r.sin();
        m.tab[2][1] = r.sin();
        m.tab[2][2] = r.cos();
        self.apply_transform(&m);
    });

    transformfun!(rotation_y; apply_rotation_y (self, r : Float) {
        let mut m = Mat::identity(4);
        m.tab[0][0] = r.cos();
        m.tab[0][2] = r.sin();
        m.tab[2][0] = -r.sin();
        m.tab[2][2] = r.cos();
        self.apply_transform(&m);
    });

    transformfun!(rotation_z; apply_rotation_z (self, r : Float) {
        let mut m = Mat::identity(4);
        m.tab[0][0] = r.cos();
        m.tab[0][1] = -r.sin();
        m.tab[1][0] = r.sin();
        m.tab[1][1] = r.cos();
        self.apply_transform(&m);
    });

    fn shearing(
        mut self,
        xy: Float,
        xz: Float,
        yx: Float,
        yz: Float,
        zx: Float,
        zy: Float,
    ) -> Self {
        let mut m = Mat::identity(4);
        m.tab[0][1] = xy;
        m.tab[0][2] = xz;
        m.tab[1][0] = yx;
        m.tab[1][2] = yz;
        m.tab[2][0] = zx;
        m.tab[2][1] = zy;
        self.apply_transform(&m);
        self
    }

    fn apply_transform(&mut self, transform: &Mat);
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use super::*;
    use crate::utils::{matrix::Mat, vec3::Vec3};

    #[test]
    fn test_translation() {
        let t = Mat::identity(4).translation(5.0, -3.0, 2.0);
        let p = Vec3::new(-3.0, 4.0, 5.0);

        assert_eq!(&t * &p, Vec3::new(2.0, 1.0, 7.0))
    }

    #[test]
    fn test_scaling() {
        let t = Mat::identity(4).scaling(2.0, 3.0, 4.0);
        let p = Vec3::new(-4.0, 6.0, 8.0);

        assert_eq!(&t * &p, Vec3::new(-8.0, 18.0, 32.0))
    }

    // Stolen
    #[test]
    fn test_scaling_matrix_x_point() {
        let transform = Mat::identity(4).scaling(2.0, 3.0, 4.0);
        let p = Vec3::new(-4.0, 6.0, 8.0);
        let expected = Vec3::new(-8.0, 18.0, 32.0);

        let res = &transform * &p;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_scaling_matrix_x_vec3() {
        let transform = Mat::identity(4).scaling(2.0, 3.0, 4.0);
        let v = Vec3::new(-4.0, 6.0, 8.0);
        let expected = Vec3::new(-8.0, 18.0, 32.0);

        let res = &transform * &v;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_inverse_scaling_matrix_x_vec3() {
        let transform = Mat::identity(4).scaling(2.0, 3.0, 4.0);
        let inverse_transform = transform.inverse();
        let v = Vec3::new(-4.0, 6.0, 8.0);
        let expected = Vec3::new(-2.0, 2.0, 2.0);

        let res = &inverse_transform * &v;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_reflection_using_scaling_matrix() {
        let transform = Mat::identity(4).scaling(-1.0, 1.0, 1.0);
        let p = Vec3::new(2.0, 3.0, 4.0);
        let expected = Vec3::new(-2.0, 3.0, 4.0);

        let res = &transform * &p;
        assert_eq!(res, expected);
    }
    // END of crime

    #[test]
    fn test_rotation_x() {
        let t_quarter = Mat::identity(4).rotation_x(PI / 4.0);
        let t_half = Mat::identity(4).rotation_x(PI / 2.0);
        let p = Vec3::new(0.0, 1.0, 0.0);

        assert_eq!(&t_half * &p, Vec3::new(0.0, 0.0, 1.0));
        assert_eq!(
            &t_quarter * &p,
            Vec3::new(0.0, 2.0f32.sqrt() / 2.0, 2.0f32.sqrt() / 2.0)
        );
    }

    #[test]
    fn test_rotation_y() {
        let t_quarter = Mat::identity(4).rotation_y(PI / 4.0);
        let t_half = Mat::identity(4).rotation_y(PI / 2.0);
        let p = Vec3::new(0.0, 0.0, 1.0);

        assert_eq!(&t_half * &p, Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(
            &t_quarter * &p,
            Vec3::new(2.0f32.sqrt() / 2.0, 0.0, 2.0f32.sqrt() / 2.0)
        );
    }

    #[test]
    fn test_rotation_z() {
        let t_quarter = Mat::identity(4).rotation_z(PI / 4.0);
        let t_half = Mat::identity(4).rotation_z(PI / 2.0);
        let p = Vec3::new(0.0, 1.0, 0.0);

        assert_eq!(&t_half * &p, Vec3::new(-1.0, 0.0, 0.0));
        assert_eq!(
            &t_quarter * &p,
            Vec3::new(2.0f32.sqrt() / -2.0, 2.0f32.sqrt() / 2.0, 0.0)
        );
    }

    #[test]
    fn test_shearing_xy() {
        let t = Mat::identity(4).shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Vec3::new(2.0, 3.0, 4.0);

        assert_eq!(&t * &p, Vec3::new(5.0, 3.0, 4.0));
    }

    #[test]
    fn test_shearing_xz() {
        let t = Mat::identity(4).shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Vec3::new(2.0, 3.0, 4.0);

        assert_eq!(&t * &p, Vec3::new(6.0, 3.0, 4.0));
    }

    #[test]
    fn test_shearing_yx() {
        let t = Mat::identity(4).shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Vec3::new(2.0, 3.0, 4.0);

        assert_eq!(&t * &p, Vec3::new(2.0, 5.0, 4.0));
    }

    #[test]
    fn test_shearing_yz() {
        let t = Mat::identity(4).shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Vec3::new(2.0, 3.0, 4.0);

        assert_eq!(&t * &p, Vec3::new(2.0, 7.0, 4.0));
    }

    #[test]
    fn test_shearing_zx() {
        let t = Mat::identity(4).shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Vec3::new(2.0, 3.0, 4.0);

        assert_eq!(&t * &p, Vec3::new(2.0, 3.0, 6.0));
    }

    #[test]
    fn test_shearing_zy() {
        let t = Mat::identity(4).shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Vec3::new(2.0, 3.0, 4.0);

        assert_eq!(&t * &p, Vec3::new(2.0, 3.0, 7.0));
    }

    #[test]
    fn test_chain() {
        let p = Vec3::new(1.0, 0.0, 1.0);
        let r = &Mat::identity(4).rotation_x(PI / 2.0);
        let s = &Mat::identity(4).scaling(5.0, 5.0, 5.0);
        let t = &Mat::identity(4).translation(10.0, 5.0, 7.0);

        let exp = Mat::identity(4)
            .translation(10.0, 5.0, 7.0)
            .scaling(5.0, 5.0, 5.0)
            .rotation_x(PI / 2.0);

        assert_eq!(&exp * &p, Vec3::new(15.0, 0.0, 7.0));
        assert_eq!(&(&(t * s) * r) * &p, Vec3::new(15.0, 0.0, 7.0));
    }
}
