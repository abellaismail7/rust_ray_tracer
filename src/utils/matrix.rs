use core::ops::Mul;

use super::vec3::{Float, Vec3, EPSILON};

#[derive(Debug)]
pub struct Mat {
    pub tab: Vec<Vec<Float>>,
    pub rows: usize,
    pub cols: usize,
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

    pub fn from_vec3(v3: &Vec3, rows: usize) -> Self {
        let mut m = Self {
            tab: vec![vec![1.0; 1]; rows],
            cols: 1,
            rows,
        };
        m.tab[0][0] = v3.x;
        m.tab[1][0] = v3.y;
        m.tab[2][0] = v3.z;
        m
    }

    pub fn identity(&self) -> Self {
        let mut m = Self {
            tab: vec![vec![0.0; self.cols]; self.cols],
            rows: self.cols,
            cols: self.cols,
        };

        for i in 0..m.rows {
            m.tab[i][i] = 1.0;
        }
        m
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
        let m1 = Mat::from_vec3(rhs, self.cols);
        let r = self * &m1;
        Vec3::new(r.tab[0][0], r.tab[1][0], r.tab[2][0])
    }
}
