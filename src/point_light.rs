use crate::color::Color;
use crate::tuple::Tuple;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PointLight {
    pub position: Tuple,
    pub intensity: Color,
    pub id: usize,
}

impl PointLight {
    #[must_use]
    pub fn new(position: Tuple, intensity: Color) -> Self {
        PointLight {
            position,
            intensity,
            id: 0,
        }
    }
}
