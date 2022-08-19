use crate::color::Color;
use crate::tuple::Tuple;

pub struct PointLight {
    pub position: Tuple,
    pub intensity: Color,
}

impl PointLight {
    #[must_use]
    pub fn new(position: Tuple, intensity: Color) -> Self {
        PointLight {
            position,
            intensity,
        }
    }
}
