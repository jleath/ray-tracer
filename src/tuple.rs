use std::ops;

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

    pub fn magnitude(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        float_near_equal(self.x, other.x)
            && float_near_equal(self.y, other.y)
            && float_near_equal(self.z, other.z)
            && float_near_equal(self.w, other.w)
    }
}

impl ops::Add for Tuple {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl ops::AddAssign for Tuple {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Sub for Tuple {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl ops::SubAssign for Tuple {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl ops::Neg for Tuple {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -(self.x),
            y: -(self.y),
            z: -(self.z),
            w: -(self.w),
        }
    }
}

impl ops::Mul<f64> for Tuple {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl ops::MulAssign<f64> for Tuple {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl ops::Div<f64> for Tuple {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl ops::DivAssign<f64> for Tuple {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
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

    #[test]
    fn addition() {
        // add vector to point produces point
        let mut a1 = Tuple::point(3.0, -2.0, 5.0);
        let a2 = Tuple::vector(-2.0, 3.0, 1.0);
        assert_eq!(a1 + a2, Tuple::point(1.0, 1.0, 6.0));

        // AddAssign trait
        a1 += a2;
        assert_eq!(a1, Tuple::point(1.0, 1.0, 6.0));
    }

    #[test]
    fn subtraction() {
        // subtract point from point produces vector
        let mut a1 = Tuple::point(3.0, 2.0, 1.0);
        let mut a2 = Tuple::point(5.0, 6.0, 7.0);
        assert_eq!(a1 - a2, Tuple::vector(-2.0, -4.0, -6.0));

        // SubAssign trait
        a1 -= a2;
        assert_eq!(a1, Tuple::vector(-2.0, -4.0, -6.0));

        // subtract vector from point produces point
        a1 = Tuple::point(3.0, 2.0, 1.0);
        a2 = Tuple::vector(5.0, 6.0, 7.0);
        assert_eq!(a1 - a2, Tuple::point(-2.0, -4.0, -6.0));

        // subtract vector from vector produces vector
        a1 = Tuple::vector(3.0, 2.0, 1.0);
        a2 = Tuple::vector(5.0, 6.0, 7.0);
        assert_eq!(a1 - a2, Tuple::vector(-2.0, -4.0, -6.0));

        // subtracting from zero vector
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let v = Tuple::vector(1.0, -2.0, 3.0);
        assert_eq!(zero - v, Tuple::vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn negation() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(-a, Tuple::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn scalar_mult() {
        let mut a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));

        a *= 3.5;
        assert_eq!(a, Tuple::new(3.5, -7.0, 10.5, -14.0));

        // mul by fraction
        a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn scalar_div() {
        let mut a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));

        a /= 2.0;
        assert_eq!(a, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn magnitude() {
        let mut v = Tuple::vector(1.0, 0.0, 0.0);
        assert!(float_near_equal(v.magnitude(), 1.0));

        v = Tuple::vector(0.0, 1.0, 0.0);
        assert!(float_near_equal(v.magnitude(), 1.0));

        v = Tuple::vector(0.0, 0.0, 1.0);
        assert!(float_near_equal(v.magnitude(), 1.0));

        v = Tuple::vector(1.0, 2.0, 3.0);
        assert!(float_near_equal(v.magnitude(), (14.0_f64).sqrt()));

        v = Tuple::vector(-1.0, -2.0, -3.0);
        assert!(float_near_equal(v.magnitude(), (14.0_f64).sqrt()));
    }

    #[test]
    fn normalize() {
        let mut v = Tuple::vector(4.0, 0.0, 0.0);
        assert_eq!(v.normalize(), Tuple::vector(1.0, 0.0, 0.0));

        v = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(
            v.normalize(),
            Tuple::vector(
                1.0 / 14_f64.sqrt(),
                2.0 / 14_f64.sqrt(),
                3.0 / 14_f64.sqrt()
            )
        );

        assert!(float_near_equal(v.normalize().magnitude(), 1.0));
    }
}
