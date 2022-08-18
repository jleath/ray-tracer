use std::ops;

use crate::float_near_equal;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub red: f64,
    pub blue: f64,
    pub green: f64,
}

impl Color {
    #[must_use]
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, blue, green }
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    pub fn red_to_int(&self) -> u8 {
        (self.red * 255.0).clamp(0.0, 255.0).round() as u8
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    pub fn blue_to_int(&self) -> u8 {
        (self.blue * 255.0).clamp(0.0, 255.0).round() as u8
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    pub fn green_to_int(&self) -> u8 {
        (self.green * 255.0).clamp(0.0, 255.0).round() as u8
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        float_near_equal(self.red, other.red)
            && float_near_equal(self.blue, other.blue)
            && float_near_equal(self.green, other.green)
    }
}

impl ops::Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red + rhs.red,
            blue: self.blue + rhs.blue,
            green: self.green + rhs.green,
        }
    }
}

impl ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Sub for Color {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red - rhs.red,
            blue: self.blue - rhs.blue,
            green: self.green - rhs.green,
        }
    }
}

impl ops::SubAssign for Color {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl ops::Mul<f64> for Color {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            red: self.red * rhs,
            blue: self.blue * rhs,
            green: self.green * rhs,
        }
    }
}

impl ops::MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl ops::Mul<Color> for Color {
    type Output = Self;
    fn mul(self, rhs: Color) -> Self::Output {
        Self {
            red: self.red * rhs.red,
            blue: self.blue * rhs.blue,
            green: self.green * rhs.green,
        }
    }
}

impl ops::MulAssign<Color> for Color {
    fn mul_assign(&mut self, rhs: Color) {
        *self = *self * rhs;
    }
}
