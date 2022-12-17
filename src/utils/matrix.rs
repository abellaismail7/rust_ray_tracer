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

    pub fn transpose(&self) -> Self {
        let mut m = Self::default(self.cols, self.rows);
        for j in 0..m.rows {
            for i in 0..m.cols {
                m.tab[i][j] = self.tab[j][i];
            }
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
}
