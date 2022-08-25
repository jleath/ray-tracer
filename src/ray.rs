use crate::tuple::Tuple;
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    #[must_use]
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        Ray { origin, direction }
    }

    #[must_use]
    pub fn position(&self, t: f64) -> Tuple {
        self.origin + self.direction * t
    }
}
