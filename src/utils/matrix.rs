use core::ops::Mul;
use std::ops::BitXor;

use crate::world::transform::Transformable;

use super::vec3::{Float, Vec3, EPSILON};

#[macro_export()]
macro_rules! at {
    ($m:expr, $rows:expr, $cols:expr) => {
        ($m.tab[($m.rows * $rows) + $cols])
    };
}


#[derive()]
pub struct Mat {
    pub rows: usize,
    pub cols: usize,
    pub tab: Box<[Float]>,
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
    pub fn new(tab: Box<[Float]>, cols: usize) -> Self {
        let cols = cols;
        let rows = tab.len() / cols;
        Self { tab, cols, rows }
    }

    pub fn default(cols: usize, rows: usize) -> Self {
        Self {
            tab: vec![0.0; cols * rows].into_boxed_slice(),
            cols,
            rows,
        }
    }

    pub fn from_vec3(v3: &Vec3, rows: usize, def: Float) -> Self {
        let mut m = Self {
            tab: vec![def; rows].into_boxed_slice(),
            cols: 1,
            rows,
        };
        at!(m, 0,0) = v3.x;
        at!(m, 1,0) = v3.y;
        at!(m, 2,0) = v3.z;
        m
    }

    pub fn identity(cols: usize) -> Self {
        let mut m = Self {
            tab: vec![0.0; cols * cols].into_boxed_slice(),
            rows: cols,
            cols,
        };

        for i in 0..cols {
            at!(m, i,i) = 1.0;
        }
        m
    }

    pub fn transpose(&self) -> Self {
        let mut m = Self::default(self.cols, self.rows);
        for j in 0..m.rows {
            for i in 0..m.cols {
                at!(m, i,j) = at!(self, j,i);
            }
        }
        m
    }

    pub fn determinant(&self) -> Float {
        if self.rows == 2 {
            return at!(self, 0,0) * at!(self,1,1) - at!(self, 0,1) * at!(self, 1,0);
        }
        let mut r = 0.0;
        for j in 0..self.cols {
            r += at!(self, 0,j) * self.cofactor(0, j);
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
                at!(m,x,y) = at!(self, i,j);
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
                at!(m, i,j) = self.cofactor(j, i) / d;
            }
        }
        m
    }

    pub fn view_transformation(from: &Vec3, to: &Vec3, up: &Vec3) -> Mat {
        let forward = (to - from).norm();
        let left = forward.cross(&up.norm());
        let true_up = left.cross(&forward);
        let orient = Mat::new(Box::new([
            left.x, left.y, left.z, 0.0,
            true_up.x, true_up.y, true_up.z, 0.0,
            -forward.x, -forward.y, -forward.z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]), 4);
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
                if (at!(self, i,j) - at!(other, i,j)).abs() > EPSILON {
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
        let mut m = Self::Output::default(rhs.cols, self.rows);
        for row in 0..m.rows {
            for col in 0..m.cols {
                // the same performace as the for loop
                // at!(m,row,col) = at!(self,row,0) * at!(rhs, 0,col)
                //     + at!(self,row,1) * at!(rhs, 1,col)
                //     + at!(self,row,2) * at!(rhs, 2,col)
                //     + at!(self,row,3) * at!(rhs, 3,col);
                for k in 0..m.rows {
                    at!(m,row,col) += at!(self,row,k) * at!(rhs, k,col);
                }
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
        //Vec3::new(r.tab[at!(0,0)], r.tab[at!(1,0)], r.tab[at!(2,0)])
        Self::Output::new(
            at!(self, 0,0) * rhs.x
                + at!(self, 0,1) * rhs.y
                + at!(self, 0,2) * rhs.z
                + at!(self, 0,3),
            at!(self, 1,0) * rhs.x
                + at!(self,1,1) * rhs.y
                + at!(self,1,2) * rhs.z
                + at!(self,1,3),
            at!(self, 2,0) * rhs.x
                + at!(self, 2,1) * rhs.y
                + at!(self, 2,2) * rhs.z
                + at!(self, 2,3),
        )
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl BitXor<&Vec3> for &Mat {
    type Output = Vec3;

    fn bitxor(self, rhs: &Vec3) -> Vec3 {
        // let m1 = Mat::from_vec3(rhs, self.cols, 0.0);
        // let r = self * &m1;
        // Vec3::new(r.tab[at!(0,0)], r.tab[at!(1,0)], r.tab[at!(2,0)])
        Self::Output::new(
            at!(self, 0,0) * rhs.x + at!(self,0,1) * rhs.y + at!(self, 0,2) * rhs.z,
            at!(self, 1,0) * rhs.x + at!(self,1,1) * rhs.y + at!(self, 1,2) * rhs.z,
            at!(self, 2,0) * rhs.x + at!(self,2,1) * rhs.y + at!(self, 2,2) * rhs.z,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_mat4_mat4() {
        let m1 = Mat::new(Box::new([
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0,
        ]), 4);
        let m2 = Mat::new(Box::new([
            -2.0, 1.0, 2.0, 3.0,
            3.0, 2.0, 1.0, -1.0,
            4.0, 3.0, 6.0, 5.0,
            1.0, 2.0, 7.0, 8.0,
        ]), 4);

        let m = &m1 * &m2;

        assert_eq!(
            m,
            Mat::new(Box::new([
                20.0, 22.0, 50.0, 48.0,
                44.0, 54.0, 114.0, 108.0,
                40.0, 58.0, 110.0, 102.0,
                16.0, 26.0, 46.0, 42.0,
            ]), 4)
        );
    }

    #[test]
    fn test_mul_mat4_vec3() {
        let m = Mat::new(Box::new([
            1.0, 2.0, 3.0, 4.0,
            2.0, 4.0, 4.0, 2.0,
            8.0, 6.0, 4.0, 1.0,
            0.0, 0.0, 0.0, 1.0,
        ]), 4);
        let v = Vec3::new(1.0, 2.0, 3.0);

        let res = &m * &v;

        assert_eq!(res, Vec3::new(18.0, 24.0, 33.0));
    }

    #[test]
    fn test_transpose() {
        let m = Mat::new(Box::new([
            1.0, 2.0, 3.0, 4.0,
            2.0, 4.0, 4.0, 2.0,
            8.0, 6.0, 4.0, 1.0,
            0.0, 0.0, 0.0, 1.0,
        ]), 4);

        assert_eq!(
            m.transpose(),
            Mat::new(Box::new([
                1.0, 2.0, 8.0, 0.0,
                2.0, 4.0, 6.0, 0.0,
                3.0, 4.0, 4.0, 0.0,
                4.0, 2.0, 1.0, 1.0,
            ]), 4)
        );
    }

    #[test]
    fn test_determinant_2x2() {
        let m = Mat::new(Box::new([1.0, 5.0, -3.0, 2.0]), 2);

        assert_eq!(m.determinant(), 17.0);
    }

    #[test]
    fn test_submatri_4x4() {
        let m = Mat::new(Box::new([
            1.0, 2.0, 3.0, 4.0,
            2.0, 4.0, 4.0, 2.0,
            8.0, 6.0, 4.0, 1.0,
            0.0, 0.0, 0.0, 1.0,
        ]), 4);
        let r = m.submatrix(0, 2);
        assert_eq!(
            r,
            Mat::new(Box::new([
                2.0, 4.0, 2.0,
                8.0, 6.0, 1.0,
                0.0, 0.0, 1.0,
            ]), 3)
        );
    }

    #[test]
    fn test_submatri_3x3() {
        let m = Mat::new(Box::new([
            2.0, 4.0, 2.0,
            8.0, 6.0, 1.0,
            0.0, 0.0, 1.0,
        ]), 3);
        let r = m.submatrix(2, 1);
        assert_eq!(r, Mat::new(Box::new([2.0, 2.0, 8.0, 1.0]), 2));
    }

    #[test]
    fn test_minor() {
        let m = Mat::new(Box::new([
            3.0, 5.0, 0.0,
            2.0, -1.0, -7.0,
            6.0, -1.0, 5.0,
        ]), 3);

        assert_eq!(m.minor(1, 0), 25.0);
    }

    #[test]
    fn test_cofactor() {
        let m = Mat::new(Box::new([
            3.0, 5.0, 0.0,
            2.0, -1.0, -7.0,
            6.0, -1.0, 5.0,
        ]), 3);

        assert_eq!(m.minor(0, 0), -12.0);
        assert_eq!(m.cofactor(0, 0), -12.0);
        assert_eq!(m.minor(1, 0), 25.0);
        assert_eq!(m.cofactor(1, 0), -25.0);
    }

    #[test]
    fn test_determinant_4x4() {
        let m = Mat::new(Box::new([
            -2.0, -8.0, 3.0, 5.0,
            -3.0, 1.0, 7.0, 3.0,
            1.0, 2.0, -9.0, 6.0,
            -6.0, 7.0, 7.0, -9.0,
        ]), 4);

        assert_eq!(m.determinant(), -4071.0);
    }

    #[test]
    fn test_inverse() {
        let m = Mat::new(Box::new([
            -5.0, 2.0, 6.0, -8.0,
            1.0, -5.0, 1.0, 8.0,
            7.0, 7.0, -6.0, -7.0,
            1.0, -3.0, 7.0, 4.0,
        ]), 4);

        let expected = Mat::new(Box::new([
            0.21805, 0.45113, 0.24060, -0.04511,
            -0.80827, -1.45677, -0.44361, 0.52068,
            -0.07895, -0.22368, -0.05263, 0.19737,
            -0.52256, -0.81391, -0.30075, 0.30639,
        ]), 4);

        assert_eq!(m.inverse(), expected);
    }

    #[test]
    fn test_inverse_1() {
        let m1 = Mat::new(Box::new([
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0,
        ]), 4);
        let m2 = Mat::new(Box::new([
            -2.0, 1.0, 2.0, 3.0,
            3.0, 2.0, 1.0, -1.0,
            4.0, 3.0, 6.0, 5.0,
            1.0, 2.0, 7.0, 8.0,
        ]), 4);

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
            Mat::new(Box::new([
                -0.50709, 0.50709, 0.67612, -2.36643,
                0.76772, 0.60609, 0.12122, -2.82843,
                -0.35857, 0.59761, -0.71714, 0.00000,
                0.00000, 0.00000, 0.00000, 1.00000,
            ]), 4)
        );
    }
}
