use core::ops::Mul;

use super::vec3::{Float, Vec3, EPSILON};

#[derive(Debug)]
pub struct Mat {
    pub tab: Vec<Vec<Float>>,
    pub rows: usize,
    pub cols: usize,
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

