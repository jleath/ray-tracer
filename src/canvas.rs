use crate::color::Color;

pub struct Canvas {
    buffer: Vec<Color>,
    width: usize,
    height: usize,
}

impl Canvas {
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            buffer: vec![Color::new(0.0, 0.0, 0.0); width * height],
            width,
            height,
        }
    }

    #[must_use]
    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.buffer[(y * self.width) + x]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, c: Color) {
        self.buffer[(y * self.width) + x] = c;
    }

    #[must_use]
    pub fn width(&self) -> usize {
        self.width
    }

    #[must_use]
    pub fn height(&self) -> usize {
        self.height
    }

    #[must_use]
    pub fn pixels(&self) -> &Vec<Color> {
        &(self.buffer)
    }
}
