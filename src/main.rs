use ray_tracer::canvas::Canvas;
use ray_tracer::color::Color;
use ray_tracer::ppm_printer::PpmPrinter;
use ray_tracer::transform::Transform;
use ray_tracer::tuple::Tuple;
use std::f64::consts::PI;

const CANVAS_WIDTH: usize = 500;
const CANVAS_HEIGHT: usize = 500;

fn main() {
    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT);
    let point_color = Color::new(1.0, 1.0, 1.0);
    let twelve_point = Tuple::point(0.0, 0.0, 1.0);
    let radius = CANVAS_WIDTH as f64 * (3.0 / 8.0);
    let mut hour_points = vec![];

    for hour in 1..=12 {
        let hour_point = Transform::new()
            .rotate_y(hour as f64 * (PI / 6.0))
            .transform(&twelve_point);
        hour_points.push(hour_point);
    }

    for point in hour_points {
        let x = (point.x * radius) + (CANVAS_WIDTH as f64 / 2.0);
        let z = (point.z * radius) + (CANVAS_HEIGHT as f64 / 2.0);
        canvas.write_pixel(x as usize, z as usize, point_color);
    }

    PpmPrinter::dump_to_file(&canvas, "clock.ppm").unwrap();
}
