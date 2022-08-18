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
    fn pixel_data(canvas: &Canvas) -> String {
        let mut pixel_string = String::new();
        let mut line_len = 0;
        let mut values_printed = 0;
        for pixel in canvas.pixels() {
            let red = pixel.red_to_int().to_string();
            let blue = pixel.blue_to_int().to_string();
            let green = pixel.green_to_int().to_string();
            if line_len + red.len() + 1 >= 70 {
                pixel_string = pixel_string.trim_end().to_string();
                pixel_string += "\n";
                line_len = 0;
            }
            line_len += red.len() + 1;
            pixel_string += &red;
            pixel_string += " ";
            if line_len + green.len() + 1 >= 70 {
                pixel_string = pixel_string.trim_end().to_string();
                pixel_string += "\n";
                line_len = 0;
            }
            line_len += green.len() + 1;
            pixel_string += &green;
            pixel_string += " ";
            if line_len + blue.len() + 1 >= 70 {
                pixel_string = pixel_string.trim_end().to_string();
                pixel_string += "\n";
                line_len = 0;
            }
            line_len += blue.len() + 1;
            pixel_string += &blue;
            if values_printed + 1 < canvas.width() {
                pixel_string += " ";
            }
            values_printed += 1;
            if values_printed == canvas.width() {
                pixel_string += "\n";
                line_len = 0;
                values_printed = 0;
            }
        }
        if !pixel_string.ends_with('\n') {
            pixel_string += "\n";
        }
        pixel_string
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
