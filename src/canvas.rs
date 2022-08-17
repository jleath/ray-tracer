use crate::color::Color;

pub struct Canvas {
    buffer: Vec<Color>,
    width: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            buffer: vec![Color::new(0.0, 0.0, 0.0); width * height],
            width,
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.buffer[(y * self.width) + x]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, c: Color) {
        self.buffer[(y * self.width) + x] = c;
    }
}
