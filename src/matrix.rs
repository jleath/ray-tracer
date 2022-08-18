use crate::float_near_equal;
use crate::tuple::*;
use std::ops;

#[derive(Debug, Clone)]
pub struct Matrix {
    buffer: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(data: &Vec<Vec<f64>>) -> Self {
        Matrix {
            buffer: data.to_owned(),
        }
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.buffer[row][col]
    }

    pub fn identity_matrix() -> Matrix {
        Matrix::new(&vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
    }
}

// Warning: Probably very slow, also relies on the matrix not being empty and having
// equal row lengths
impl ops::Mul<Matrix> for Matrix {
    type Output = Self;
    fn mul(self, rhs: Matrix) -> Self::Output {
        let mut result = vec![];
        let width = self.buffer[0].len();
        let height = self.buffer.len();

        for row in 0..height {
            let mut result_row = vec![];
            for col in 0..width {
                let mut cur = 0.0;
                for i in 0..height {
                    cur += self.buffer[row][i] * rhs.buffer[i][col];
                }
                result_row.push(cur);
            }
            result.push(result_row);
        }
        Matrix::new(&result)
    }
}

impl ops::Mul<Tuple> for Matrix {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Self::Output {
        let x = self.buffer[0][0] * rhs.x
            + self.buffer[0][1] * rhs.y
            + self.buffer[0][2] * rhs.z
            + self.buffer[0][3] * rhs.w;
        let y = self.buffer[1][0] * rhs.x
            + self.buffer[1][1] * rhs.y
            + self.buffer[1][2] * rhs.z
            + self.buffer[1][3] * rhs.w;
        let z = self.buffer[2][0] * rhs.x
            + self.buffer[2][1] * rhs.y
            + self.buffer[2][2] * rhs.z
            + self.buffer[2][3] * rhs.w;
        let w = self.buffer[3][0] * rhs.x
            + self.buffer[3][1] * rhs.y
            + self.buffer[3][2] * rhs.z
            + self.buffer[3][3] * rhs.w;
        Tuple::new(x, y, z, w)
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.buffer.len() != other.buffer.len() {
            return false;
        }
        for y in 0..self.buffer.len() {
            let row_len = self.buffer[y].len();
            if row_len != other.buffer[y].len() {
                return false;
            }
            for x in 0..self.buffer[y].len() {
                if !float_near_equal(self.buffer[y][x], other.buffer[y][x]) {
                    return false;
                }
            }
        }
        true
    }
}
