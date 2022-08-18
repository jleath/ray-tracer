use crate::matrix::*;
use crate::tuple::*;

pub struct Transform {
    matrix: Matrix,
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            matrix: Matrix::identity_matrix(),
        }
    }

    pub fn transform(&self, t: &Tuple) -> Tuple {
        self.matrix.tuple_multiply(t)
    }
    pub fn translate(mut self, x: f64, y: f64, z: f64) -> Self {
        let mut matrix = Matrix::identity_matrix();
        matrix.set(0, 3, x);
        matrix.set(1, 3, y);
        matrix.set(2, 3, z);
        self.matrix = matrix.matrix_multiply(&self.matrix);
        self
    }

    pub fn scale(mut self, x: f64, y: f64, z: f64) -> Self {
        let mut matrix = Matrix::identity_matrix();
        matrix.set(0, 0, x);
        matrix.set(1, 1, y);
        matrix.set(2, 2, z);
        self.matrix = matrix.matrix_multiply(&self.matrix);
        self
    }

    pub fn rotate_x(mut self, r: f64) -> Self {
        let mut matrix = Matrix::identity_matrix();
        matrix.set(1, 1, r.cos());
        matrix.set(1, 2, -(r.sin()));
        matrix.set(2, 1, r.sin());
        matrix.set(2, 2, r.cos());
        self.matrix = matrix.matrix_multiply(&self.matrix);
        self
    }

    pub fn rotate_y(mut self, r: f64) -> Self {
        let mut matrix = Matrix::identity_matrix();
        matrix.set(0, 0, r.cos());
        matrix.set(0, 2, r.sin());
        matrix.set(2, 0, -(r.sin()));
        matrix.set(2, 2, r.cos());
        self.matrix = matrix.matrix_multiply(&self.matrix);
        self
    }

    pub fn rotate_z(mut self, r: f64) -> Self {
        let mut matrix = Matrix::identity_matrix();
        matrix.set(0, 0, r.cos());
        matrix.set(0, 1, -(r.sin()));
        matrix.set(1, 0, r.sin());
        matrix.set(1, 1, r.cos());
        self.matrix = matrix.matrix_multiply(&self.matrix);
        self
    }

    pub fn shear(mut self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        let mut matrix = Matrix::identity_matrix();
        matrix.set(0, 1, xy);
        matrix.set(0, 2, xz);
        matrix.set(1, 0, yx);
        matrix.set(1, 2, yz);
        matrix.set(2, 0, zx);
        matrix.set(2, 1, zy);
        self.matrix = matrix.matrix_multiply(&self.matrix);
        self
    }

    pub fn inverse(mut self) -> Self {
        self.matrix = self.matrix.inverse();
        self
    }
}
