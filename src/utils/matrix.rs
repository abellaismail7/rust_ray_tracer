use core::ops::Mul;
use std::ops::BitXor;

use crate::world::transform::Transformable;

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
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> Float {
        if (row + col) % 2 == 1 {
            -self.minor(row, col)
        } else {
            self.minor(row, col)
        }
    }

    pub fn inverse(&self) -> Self {
        let d = self.determinant();
        if d == 0.0 {
            panic!("Matrix is non inversable")
        }
        let mut m = Mat::default(self.cols, self.rows);
        for i in 0..m.rows {
            for j in 0..m.cols {
                m.tab[i][j] = self.cofactor(j, i) / d;
            }
        }
        m
    }

    pub fn view_transformation(from: &Vec3, to: &Vec3, up: &Vec3) -> Mat {
        let forward = (to - from).norm();
        let left = forward.cross(&up.norm());
        let true_up = left.cross(&forward);
        let orient = Mat::new(vec![
            vec![left.x, left.y, left.z, 0.0],
            vec![true_up.x, true_up.y, true_up.z, 0.0],
            vec![-forward.x, -forward.y, -forward.z, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);
        orient.translation(-from.x, -from.y, -from.z)
    }
}

impl Transformable for Mat {
    fn apply_transform(&mut self, transform: &Mat) {
        let r = &*self;
        self.tab = (r * transform).tab;
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
        for row in 0..m.rows {
            for col in 0..m.cols {
                m.tab[row][col] = self.tab[row][0] * rhs.tab[0][col]
                    + self.tab[row][1] * rhs.tab[1][col]
                    + self.tab[row][2] * rhs.tab[2][col]
                    + self.tab[row][3] * rhs.tab[3][col];
                // for k in 0..m.rows {
                //     m.tab[i][j] += self.tab[i][k] * rhs.tab[k][j];
                // }
            }
        }
        m
    }
}

impl Mul<&Vec3> for &Mat {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Vec3 {
        //let m1 = Mat::from_vec3(rhs, self.cols, 1.0);
        //let r = self * &m1;
        //Vec3::new(r.tab[0][0], r.tab[1][0], r.tab[2][0])
        Self::Output::new(
            self.tab[0][0] * rhs.x
                + self.tab[0][1] * rhs.y
                + self.tab[0][2] * rhs.z
                + self.tab[0][3],
            self.tab[1][0] * rhs.x
                + self.tab[1][1] * rhs.y
                + self.tab[1][2] * rhs.z
                + self.tab[1][3],
            self.tab[2][0] * rhs.x
                + self.tab[2][1] * rhs.y
                + self.tab[2][2] * rhs.z
                + self.tab[2][3],
        )
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl BitXor<&Vec3> for &Mat {
    type Output = Vec3;

    fn bitxor(self, rhs: &Vec3) -> Vec3 {
        // let m1 = Mat::from_vec3(rhs, self.cols, 0.0);
        // let r = self * &m1;
        // Vec3::new(r.tab[0][0], r.tab[1][0], r.tab[2][0])
        Self::Output::new(
            self.tab[0][0] * rhs.x + self.tab[0][1] * rhs.y + self.tab[0][2] * rhs.z,
            self.tab[1][0] * rhs.x + self.tab[1][1] * rhs.y + self.tab[1][2] * rhs.z,
            self.tab[2][0] * rhs.x + self.tab[2][1] * rhs.y + self.tab[2][2] * rhs.z,
        )
    }
}

#[cfg(test)]
mod tests {
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
    fn test_inverse_1() {
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

        let res = &m1 * &m2;

        assert_eq!(&res * &m2.inverse(), m1);
    }

    #[test]
    fn test_view_tranfromation_1() {
        let from = Vec3::new(0.0, 0.0, 0.0);
        let to = Vec3::new(0.0, 0.0, -1.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let m = Mat::view_transformation(&from, &to, &up);

        assert_eq!(m, Mat::identity(4));
    }

    #[test]
    fn test_view_tranfromation_2() {
        let from = Vec3::new(1.0, 3.0, 2.0);
        let to = Vec3::new(4.0, -2.0, 8.0);
        let up = Vec3::new(1.0, 1.0, 0.0);
        let m = Mat::view_transformation(&from, &to, &up);

        assert_eq!(
            m,
            Mat::new(vec![
                vec![-0.50709, 0.50709, 0.67612, -2.36643],
                vec![0.76772, 0.60609, 0.12122, -2.82843],
                vec![-0.35857, 0.59761, -0.71714, 0.00000],
                vec![0.00000, 0.00000, 0.00000, 1.00000],
            ])
        );
    }
}
