use crate::canvas::Canvas;
use std::fs::File;
use std::io::Write;

pub struct PpmPrinter;

impl PpmPrinter {
    /// # Errors
    ///
    /// Will return an error if the file indicated by `filepath` cannot be opened or
    /// written to.
    pub fn dump_to_file(canvas: &Canvas, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let header = PpmPrinter::header(canvas);
        let data = PpmPrinter::pixel_data(canvas);
        let mut file = File::create(filepath)?;
        file.write_all((header + "\n" + data.as_str()).as_bytes())?;
        Ok(())
    }
    fn header(canvas: &Canvas) -> String {
        format!("P3\n{} {}\n255", canvas.width(), canvas.height())
    }

    // TODO: Clean this up
    /// # Panics
    ///
    /// Panics if the pixel data cannot be converted into PPM format
    #[must_use]
    pub fn pixel_data(canvas: &Canvas) -> String {
        let mut data_str = String::new();
        for row in 0..canvas.height() {
            let mut color_data = Vec::with_capacity(canvas.width());
            for col in 0..canvas.width() {
                let pixel = canvas.pixel_at(col, row);
                color_data.push(format!(
                    "{} {} {}",
                    pixel.red_to_int(),
                    pixel.green_to_int(),
                    pixel.blue_to_int()
                ));
            }
            let mut bytes = color_data.join(" ").into_bytes();
            let limit = 70;
            let mut start = 0;
            while bytes.len() - start >= limit {
                let mut i = start + limit;
                while i > 0 && bytes[i] != b' ' {
                    i -= 1;
                }
                bytes[i] = b'\n';
                start = i + 1;
            }
            data_str.push_str(std::str::from_utf8(&bytes).unwrap());
            data_str.push('\n');
        }
        data_str
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::color::Color;

    #[test]
    fn test_header() {
        let c = Canvas::new(5, 3);
        assert_eq!(PpmPrinter::header(&c), "P3\n5 3\n255");
    }

    #[test]
    fn pixel_data() {
        let mut c = Canvas::new(5, 3);
        c.write_pixel(0, 0, Color::new(1.5, 0.0, 0.0));
        c.write_pixel(2, 1, Color::new(0.0, 0.5, 0.0));
        c.write_pixel(4, 2, Color::new(-0.5, 0.0, 1.0));

        let mut expected = "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n\
                              0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n\
                              0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n";

        assert_eq!(PpmPrinter::pixel_data(&c), expected);

        c = Canvas::new(10, 2);
        for y in 0..c.height() {
            for x in 0..c.width() {
                c.write_pixel(x, y, Color::new(1.0, 0.8, 0.6));
            }
        }
        expected = "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
                    153 255 204 153 255 204 153 255 204 153 255 204 153\n\
                    255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204\n\
                    153 255 204 153 255 204 153 255 204 153 255 204 153\n";

        assert_eq!(PpmPrinter::pixel_data(&c), expected);
    }
}
