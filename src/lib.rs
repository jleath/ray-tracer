#![warn(clippy::pedantic)]

pub mod camera;
pub mod canvas;
pub mod color;
pub mod intersection;
pub mod material;
pub mod matrix;
pub mod pattern;
pub mod point_light;
pub mod ppm_printer;
pub mod ray;
pub mod shape;
pub mod transform;
pub mod tuple;
pub mod world;

pub const EPSILON: f64 = 0.00001;
pub const MAX_REFLECT_DEPTH: i32 = 5;

pub const BLACK: color::Color = color::Color {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
};

pub const WHITE: color::Color = color::Color {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
};

#[must_use]
pub fn float_near_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}
