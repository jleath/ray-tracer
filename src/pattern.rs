use crate::color::Color;
use crate::shape::Shape;
use crate::transform::Transform;
use crate::tuple::Tuple;
use crate::EPSILON;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(clippy::module_name_repetitions)]
pub enum PatternType {
    Stripes,
    Gradient,
    Rings,
    Checkered,
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
    pub fn gradient(a: Color, b: Color) -> Pattern {
        Pattern {
            a,
            b,
            kind: PatternType::Gradient,
            transform: Transform::new(),
        }
    }

    #[must_use]
    pub fn rings(a: Color, b: Color) -> Pattern {
        Pattern {
            a,
            b,
            kind: PatternType::Rings,
            transform: Transform::new(),
        }
    }

    #[must_use]
    pub fn checkered(a: Color, b: Color) -> Pattern {
        Pattern {
            a,
            b,
            kind: PatternType::Checkered,
            transform: Transform::new(),
        }
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn color_at(&self, pos: Tuple) -> Color {
        match self.kind {
            PatternType::Stripes => {
                if (pos.x.floor()) as i32 % 2 == 0 {
                    self.a
                } else {
                    self.b
                }
            }
            PatternType::Gradient => {
                let distance = self.b - self.a;
                let fraction = pos.x - pos.x.floor();
                self.a + distance * fraction
            }
            PatternType::Rings => {
                let x_squared = pos.x * pos.x;
                let z_squared = pos.z * pos.z;
                if (x_squared + z_squared).sqrt().floor() as i32 % 2 == 0 {
                    self.a
                } else {
                    self.b
                }
            }
            PatternType::Checkered => {
                let mut sum = 0.0;
                if (pos.x.ceil() - pos.x).abs() < EPSILON {
                    sum += pos.x.ceil();
                } else {
                    sum += pos.x.floor();
                }
                if (pos.y.ceil() - pos.y).abs() < EPSILON {
                    sum += pos.y.ceil();
                } else {
                    sum += pos.y.floor();
                }
                if (pos.z.ceil() - pos.z).abs() < EPSILON {
                    sum += pos.z.ceil();
                } else {
                    sum += pos.z.floor();
                }
                if sum as i64 % 2 == 0 {
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
