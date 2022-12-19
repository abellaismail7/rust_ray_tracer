use core::ops::Mul;

use super::vec3::{Float, Vec3, EPSILON};

#[derive()]
pub struct Mat {
    pub tab: Vec<Vec<Float>>,
    pub rows: usize,
    pub cols: usize,
}

impl std::fmt::Debug for Mat {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .tab
            .iter()
            .map(|v| format!("\t{:?}", v))
            .collect::<Vec<String>>()
            .join("\n");

        write!(fmt, "Matrix{}x{}[\n{}\n]", self.rows, self.cols, s)
    }
}

impl Mat {
    pub fn new(tab: Vec<Vec<Float>>) -> Self {
        let cols = tab[0].len();
        let rows = tab.len();
        Self { tab, cols, rows }
    }

    pub fn default(cols: usize, rows: usize) -> Self {
        Self {
            tab: vec![vec![0.0; cols]; rows],
            cols,
            rows,
        }
    }

    pub fn from_vec3(v3: &Vec3, rows: usize, def: Float) -> Self {
        let mut m = Self {
            tab: vec![vec![def; 1]; rows],
            cols: 1,
            rows,
        };
        m.tab[0][0] = v3.x;
        m.tab[1][0] = v3.y;
        m.tab[2][0] = v3.z;
        m
    }

    pub fn identity(cols: usize) -> Self {
        let mut m = Self {
            tab: vec![vec![0.0; cols]; cols],
            rows: cols,
            cols,
        };

        for i in 0..cols {
            m.tab[i][i] = 1.0;
        }
        m
    }

    pub fn transpose(&self) -> Self {
        let mut m = Self::default(self.cols, self.rows);
        for j in 0..m.rows {
            for i in 0..m.cols {
                m.tab[i][j] = self.tab[j][i];
            }
        }
        m
    }

    pub fn determinant(&self) -> Float {
        if self.rows == 2 {
            let tab = &self.tab;
            return tab[0][0] * tab[1][1] - tab[0][1] * tab[1][0];
        }
        let mut r = 0.0;
        for j in 0..self.cols {
            r += self.tab[0][j] * self.cofactor(0, j);
        }
        r
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Self {
        let mut m = Mat::default(self.cols - 1, self.rows - 1);
        let mut x = 0;
        for i in 0..self.rows {
            if i == row {
                continue;
            }
            let mut y = 0;
            for j in 0..self.cols {
                if j == col {
                    continue;
                }
                m.tab[x][y] = self.tab[i][j];
                y += 1;
            }
            x += 1;
        }
        m
    }

    pub fn minor(&self, row: usize, col: usize) -> Float {
        let sub = self.submatrix(row, col);
        sub.determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> Float {
        if (row + col) % 2 == 1 {
            -self.minor(row, col)
        } else {
            self.minor(row, col)
        }
    }

    pub fn inverse(&self) -> Self {
        if self.determinant() == 0.0 {
            panic!("Matrix is non inversable")
        }
        let mut m = Mat::default(self.cols, self.rows);
        let d = self.determinant();
        for i in 0..m.rows {
            for j in 0..m.cols {
                m.tab[i][j] = self.cofactor(j, i) / d;
            }
        }
        m
    }

    pub fn translation(&self, x: Float, y: Float, z: Float) -> Self {
        let mut m = Mat::identity(4);
        m.tab[0][3] = x;
        m.tab[1][3] = y;
        m.tab[2][3] = z;
        self * &m
    }

    pub fn scaling(&self, x: Float, y: Float, z: Float) -> Self {
        let mut m = Mat::identity(4);
        m.tab[0][0] = x;
        m.tab[1][1] = y;
        m.tab[2][2] = z;
        self * &m
    }

    pub fn rotation_x(&self, r: Float) -> Self {
        let mut m = Mat::identity(4);
        m.tab[1][1] = r.cos();
        m.tab[1][2] = -r.sin();
        m.tab[2][1] = r.sin();
        m.tab[2][2] = r.cos();
        self * &m
    }

    pub fn rotation_y(&self, r: Float) -> Self {
        let mut m = Mat::identity(4);
        m.tab[0][0] = r.cos();
        m.tab[0][2] = r.sin();
        m.tab[2][0] = -r.sin();
        m.tab[2][2] = r.cos();
        self * &m
    }

    pub fn rotation_z(&self, r: Float) -> Self {
        let mut m = Mat::identity(4);
        m.tab[0][0] = r.cos();
        m.tab[0][1] = -r.sin();
        m.tab[1][0] = r.sin();
        m.tab[1][1] = r.cos();
        self * &m
    }

    pub fn shearing(
        &self,
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
        self * &m
    }
}

impl PartialEq for Mat {
    fn eq(&self, other: &Self) -> bool {
        if self.cols != other.cols && self.rows != other.rows {
            return false;
        }
        for i in 0..self.rows {
            for j in 0..self.cols {
                if (self.tab[i][j] - other.tab[i][j]).abs() > EPSILON {
                    return false;
                }
            }
        }
        true
    }
}

/*
 * r1-c1 * r2-c2 => r1-c2 where c1 == r2
 */
impl Mul for &Mat {
    type Output = Mat;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.cols != rhs.rows {
            panic!("invalid matrix multiplcation");
        }
        let mut m = Self::Output::default(rhs.cols, self.rows);
        for i in 0..m.rows {
            for j in 0..m.cols {
                for k in 0..m.rows {
                    m.tab[i][j] += self.tab[i][k] * rhs.tab[k][j];
                }
            }
        }
        m
    }
}

impl Mul<&Vec3> for &Mat {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Vec3 {
        let m1 = Mat::from_vec3(rhs, self.cols, 1.0);
        let r = self * &m1;
        Vec3::new(r.tab[0][0], r.tab[1][0], r.tab[2][0])
    }
}


impl Mul<&Vec3> for &&Mat {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Vec3 {
        let m1 = Mat::from_vec3(rhs, self.cols, 0.0);
        let r = *self * &m1;
        Vec3::new(r.tab[0][0], r.tab[1][0], r.tab[2][0])
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use super::*;

    #[test]
    fn test_mul_mat4_mat4() {
        let m1 = Mat::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        let m2 = Mat::new(vec![
            vec![-2.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0],
        ]);

        let m = &m1 * &m2;

        assert_eq!(
            m,
            Mat::new(vec![
                vec![20.0, 22.0, 50.0, 48.0],
                vec![44.0, 54.0, 114.0, 108.0],
                vec![40.0, 58.0, 110.0, 102.0],
                vec![16.0, 26.0, 46.0, 42.0],
            ])
        );
    }

    #[test]
    fn test_mul_mat4_vec3() {
        let m = Mat::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);
        let v = Vec3::new(1.0, 2.0, 3.0);

        let res = &m * &v;

        assert_eq!(res, Vec3::new(18.0, 24.0, 33.0));
    }

    #[test]
    fn test_transpose() {
        let m = Mat::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);

        assert_eq!(
            m.transpose(),
            Mat::new(vec![
                vec![1.0, 2.0, 8.0, 0.0],
                vec![2.0, 4.0, 6.0, 0.0],
                vec![3.0, 4.0, 4.0, 0.0],
                vec![4.0, 2.0, 1.0, 1.0],
            ])
        );
    }

    #[test]
    fn test_determinant_2x2() {
        let m = Mat::new(vec![vec![1.0, 5.0], vec![-3.0, 2.0]]);

        assert_eq!(m.determinant(), 17.0);
    }

    #[test]
    fn test_submatri_4x4() {
        let m = Mat::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);
        let r = m.submatrix(0, 2);
        assert_eq!(
            r,
            Mat::new(vec![
                vec![2.0, 4.0, 2.0],
                vec![8.0, 6.0, 1.0],
                vec![0.0, 0.0, 1.0],
            ])
        );
    }

    #[test]
    fn test_submatri_3x3() {
        let m = Mat::new(vec![
            vec![2.0, 4.0, 2.0],
            vec![8.0, 6.0, 1.0],
            vec![0.0, 0.0, 1.0],
        ]);
        let r = m.submatrix(2, 1);
        assert_eq!(r, Mat::new(vec![vec![2.0, 2.0], vec![8.0, 1.0],]));
    }

    #[test]
    fn test_minor() {
        let m = Mat::new(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ]);

        assert_eq!(m.minor(1, 0), 25.0);
    }

    #[test]
    fn test_cofactor() {
        let m = Mat::new(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ]);

        assert_eq!(m.minor(0, 0), -12.0);
        assert_eq!(m.cofactor(0, 0), -12.0);
        assert_eq!(m.minor(1, 0), 25.0);
        assert_eq!(m.cofactor(1, 0), -25.0);
    }

    #[test]
    fn test_determinant_4x4() {
        let m = Mat::new(vec![
            vec![-2.0, -8.0, 3.0, 5.0],
            vec![-3.0, 1.0, 7.0, 3.0],
            vec![1.0, 2.0, -9.0, 6.0],
            vec![-6.0, 7.0, 7.0, -9.0],
        ]);

        assert_eq!(m.determinant(), -4071.0);
    }

    #[test]
    fn test_inverse() {
        let m = Mat::new(vec![
            vec![-5.0, 2.0, 6.0, -8.0],
            vec![1.0, -5.0, 1.0, 8.0],
            vec![7.0, 7.0, -6.0, -7.0],
            vec![1.0, -3.0, 7.0, 4.0],
        ]);

        let expected = Mat::new(vec![
            vec![0.21805, 0.45113, 0.24060, -0.04511],
            vec![-0.80827, -1.45677, -0.44361, 0.52068],
            vec![-0.07895, -0.22368, -0.05263, 0.19737],
            vec![-0.52256, -0.81391, -0.30075, 0.30639],
        ]);

        assert_eq!(m.inverse(), expected);
    }

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
