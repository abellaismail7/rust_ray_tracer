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

