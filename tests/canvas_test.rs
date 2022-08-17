use ray_tracer::canvas::*;
use ray_tracer::color::*;

#[test]
fn init() {
    let width = 10;
    let height = 20;

    let c = Canvas::new(width, height);

    for n in 0..height {
        for m in 0..width {
            assert_eq!(c.pixel_at(m, n), Color::new(0.0, 0.0, 0.0));
        }
    }
}

#[test]
fn write_pixel() {
    let mut c = Canvas::new(10, 20);
    let red = Color::new(1.0, 0.0, 0.0);
    c.write_pixel(2, 3, red);
    assert_eq!(c.pixel_at(2, 3), red);
}
