use crate::color::Color;
use crate::shape::Shape;
use crate::transform::Transform;
use crate::tuple::Tuple;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PatternType {
    Stripes,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pattern {
    a: Color,
    b: Color,
    kind: PatternType,
    transform: Transform,
}

impl Pattern {
    #[must_use]
    pub fn stripes(a: Color, b: Color) -> Pattern {
        Pattern {
            a,
            b,
            kind: PatternType::Stripes,
            transform: Transform::new(),
        }
    }

    #[must_use]
    pub fn color_at(&self, pos: Tuple) -> Color {
        match self.kind {
            PatternType::Stripes => {
                if (pos.x.floor()) as i32 % 2 == 0 {
                    self.a
                } else {
                    self.b
                }
            }
        }
    }

    #[must_use]
    pub fn color_at_object(&self, object: &Shape, pos: Tuple) -> Color {
        let obj_transform = object.transform().clone().inverse();
        let object_point = obj_transform.transform(&pos);
        let pattern_transform = self.transform.clone().inverse();
        let pattern_point = pattern_transform.transform(&object_point);

        self.color_at(pattern_point)
    }

    pub fn scale(&mut self, x: f64, y: f64, z: f64) {
        self.transform = self.transform.clone().scale(x, y, z);
    }

    pub fn translate(&mut self, x: f64, y: f64, z: f64) {
        self.transform = self.transform.clone().translate(x, y, z);
    }

    pub fn rotate_x(&mut self, r: f64) {
        self.transform = self.transform.clone().rotate_x(r);
    }

    pub fn rotate_z(&mut self, r: f64) {
        self.transform = self.transform.clone().rotate_z(r);
    }

    pub fn rotate_y(&mut self, r: f64) {
        self.transform = self.transform.clone().rotate_y(r);
    }

    pub fn shear(&mut self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) {
        self.transform = self.transform.clone().shear(xy, xz, yx, yz, zx, zy);
    }
}
