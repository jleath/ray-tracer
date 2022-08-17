fn float_near_equal(a: f64, b: f64) -> bool {
    (a - b).abs() < f64::EPSILON
}

#[derive(Copy, Clone, Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        float_near_equal(self.x, other.x)
            && float_near_equal(self.y, other.y)
            && float_near_equal(self.z, other.z)
            && float_near_equal(self.w, other.w)
    }
}

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Tuple { x, y, z, w }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Tuple::new(x, y, z, 1.0)
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Tuple::new(x, y, z, 0.0)
    }

    pub fn is_point(&self) -> bool {
        float_near_equal(self.w, 1.0)
    }

    pub fn is_vector(&self) -> bool {
        float_near_equal(self.w, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_init() {
        let point = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert!(float_near_equal(point.x, 4.3));
        assert!(float_near_equal(point.y, -4.2));
        assert!(float_near_equal(point.z, 3.1));
        assert!(float_near_equal(point.w, 1.0));
        assert!(point.is_point());
        assert!(!point.is_vector());

        let vector = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert!(vector.is_vector());
        assert!(!vector.is_point());
    }

    #[test]
    fn point_init() {
        let point = Tuple::point(4.0, -4.0, 3.0);
        assert_eq!(point, Tuple::new(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn vector_init() {
        let vector = Tuple::vector(4.0, -4.0, 3.0);
        assert_eq!(vector, Tuple::new(4.0, -4.0, 3.0, 0.0));
    }
}
