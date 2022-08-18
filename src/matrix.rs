use crate::float_near_equal;
use crate::tuple::*;

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

    pub fn set(&mut self, row: usize, col: usize, val: f64) {
        self.buffer[row][col] = val;
    }

    pub fn identity_matrix() -> Matrix {
        Matrix::new(&vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn transpose(&self) -> Matrix {
        let mut result = vec![];
        for y in 0..self.buffer.len() {
            result.push(vec![]);
            for x in 0..self.buffer[y].len() {
                result[y].push(self.buffer[x][y]);
            }
        }

        Matrix::new(&result)
    }

    pub fn determinant(&self) -> f64 {
        if self.buffer.len() == 2 {
            (self.buffer[0][0] * self.buffer[1][1]) - (self.buffer[0][1] * self.buffer[1][0])
        } else {
            let first_row = &self.buffer[0];
            let mut d = 0.0;
            for (idx, val) in first_row.iter().enumerate() {
                d += self.cofactor(0, idx) * val;
            }
            d
        }
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix {
        let mut result = vec![];
        for y in 0..self.buffer.len() {
            if y == row {
                continue;
            }
            let mut result_row = vec![];
            for x in 0..self.buffer[y].len() {
                if x == col {
                    continue;
                }
                result_row.push(self.buffer[y][x]);
            }
            result.push(result_row);
        }
        Matrix::new(&result)
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let m = self.minor(row, col);
        if (row + col) % 2 == 0 {
            m
        } else {
            -m
        }
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Matrix {
        assert!(self.is_invertible());

        let mut result = self.buffer.clone();

        for row in 0..self.buffer.len() {
            for col in 0..self.buffer[row].len() {
                let c = self.cofactor(row, col);
                result[col][row] = c / self.determinant();
            }
        }
        Matrix::new(&result)
    }

    pub fn matrix_multiply(&self, rhs: &Matrix) -> Matrix {
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

    pub fn tuple_multiply(&self, rhs: &Tuple) -> Tuple {
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
