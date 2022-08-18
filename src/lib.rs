#![warn(clippy::pedantic)]

pub mod canvas;
pub mod color;
pub mod matrix;
pub mod ppm_printer;
pub mod ray;
pub mod transform;
pub mod tuple;

#[must_use]
pub fn float_near_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < f64::EPSILON
}
