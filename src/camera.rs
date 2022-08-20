use crate::canvas::Canvas;
use crate::ray::Ray;
use crate::transform::Transform;
use crate::tuple::Tuple;
use crate::world::World;
use std::io::{stdout, Write};

#[derive(Clone, Debug, PartialEq)]
pub struct Camera {
    pub hsize: f64,
    pub vsize: f64,
    pub half_width: f64,
    pub half_height: f64,
    pub field_of_view: f64,
    pub pixel_size: f64,
    pub transform: Transform,
}

impl Camera {
    #[must_use]
    #[allow(clippy::similar_names)]
    pub fn new(hsize: f64, vsize: f64, field_of_view: f64) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize / vsize;

        let mut c = Camera {
            hsize,
            vsize,
            field_of_view,
            transform: Transform::new(),
            half_width: half_view * aspect,
            half_height: half_view,
            pixel_size: 0.0,
        };

        if aspect >= 1.0 {
            c.half_width = half_view;
            c.half_height = half_view / aspect;
        }
        c.pixel_size = (c.half_width * 2.0) / c.hsize;
        c
    }

    #[must_use]
    pub fn ray_for_pixel(&self, px: f64, py: f64) -> Ray {
        let xoffset = (px + 0.5) * self.pixel_size;
        let yoffset = (py + 0.5) * self.pixel_size;
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let t = self.transform.clone().inverse();
        let pixel = t.transform(&Tuple::point(world_x, world_y, -1.0));
        let origin = t.transform(&Tuple::point(0.0, 0.0, 0.0));
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    /// # Panics
    ///
    /// Will panic if writing to stdout fails
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    pub fn render(&self, world: &World) -> Canvas {
        let num_pixels = (self.hsize * self.vsize) as usize;
        let mut pixels_colored: usize = 0;
        let mut progress_string = String::new();
        let mut image = Canvas::new(self.hsize as usize, self.vsize as usize);
        println!("generating data for {} pixels", num_pixels);
        for y in 0..self.vsize as usize {
            for x in 0..self.hsize as usize {
                let r = self.ray_for_pixel(x as f64, y as f64);
                let color = world.color_at(&r);
                pixels_colored += 1;
                let percent_done = (pixels_colored * 100 / num_pixels * 100) / 100;
                while percent_done / 2 > progress_string.len() {
                    progress_string += "=";
                    print!("\r[{}>] {}%", progress_string, percent_done);
                    stdout().flush().unwrap();
                }

                image.write_pixel(x, y, color);
            }
        }
        println!();
        image
    }
}
