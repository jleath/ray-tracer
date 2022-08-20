#![warn(clippy::pedantic)]

pub mod camera;
pub mod canvas;
pub mod color;
pub mod intersection;
pub mod material;
pub mod matrix;
pub mod point_light;
pub mod ppm_printer;
pub mod ray;
pub mod sphere;
pub mod transform;
pub mod tuple;
pub mod world;

pub const EPSILON: f64 = 0.00001;

#[must_use]
pub fn float_near_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}
