pub mod tuple;

pub fn float_near_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < f64::EPSILON
}
