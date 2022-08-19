use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::tuple::Tuple;

#[derive(Clone, Debug, PartialEq)]
pub struct Transform {
    matrix: Matrix,
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}

impl Transform {
    #[must_use]
    pub fn new() -> Self {
        Transform {
            matrix: Matrix::identity_matrix(),
        }
    }

    #[must_use]
    pub fn transform(&self, t: &Tuple) -> Tuple {
        self.matrix.tuple_multiply(t)
    }

    #[must_use]
    pub fn transform_ray(&self, r: &Ray) -> Ray {
        Ray {
            origin: self.transform(&r.origin),
            direction: self.transform(&r.direction),
        }
    }

    #[must_use]
    pub fn translate(mut self, x: f64, y: f64, z: f64) -> Self {
        let mut matrix = Matrix::identity_matrix();
        matrix.set(0, 3, x);
        matrix.set(1, 3, y);
        matrix.set(2, 3, z);
        self.matrix = matrix.matrix_multiply(&self.matrix);
        self
    }

    #[must_use]
    pub fn scale(mut self, x: f64, y: f64, z: f64) -> Self {
        let mut matrix = Matrix::identity_matrix();
        matrix.set(0, 0, x);
        matrix.set(1, 1, y);
        matrix.set(2, 2, z);
        self.matrix = matrix.matrix_multiply(&self.matrix);
        self
    }

    #[must_use]
    pub fn rotate_x(mut self, r: f64) -> Self {
        let mut matrix = Matrix::identity_matrix();
        matrix.set(1, 1, r.cos());
        matrix.set(1, 2, -(r.sin()));
        matrix.set(2, 1, r.sin());
        matrix.set(2, 2, r.cos());
        self.matrix = matrix.matrix_multiply(&self.matrix);
        self
    }

    #[must_use]
    pub fn rotate_y(mut self, r: f64) -> Self {
        let mut matrix = Matrix::identity_matrix();
        matrix.set(0, 0, r.cos());
        matrix.set(0, 2, r.sin());
        matrix.set(2, 0, -(r.sin()));
        matrix.set(2, 2, r.cos());
        self.matrix = matrix.matrix_multiply(&self.matrix);
        self
    }

    #[must_use]
    pub fn rotate_z(mut self, r: f64) -> Self {
        let mut matrix = Matrix::identity_matrix();
        matrix.set(0, 0, r.cos());
        matrix.set(0, 1, -(r.sin()));
        matrix.set(1, 0, r.sin());
        matrix.set(1, 1, r.cos());
        self.matrix = matrix.matrix_multiply(&self.matrix);
        self
    }

    #[must_use]
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

    #[must_use]
    pub fn inverse(mut self) -> Self {
        self.matrix = self.matrix.inverse();
        self
    }
}
