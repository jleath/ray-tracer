use ray_tracer::canvas::Canvas;
use ray_tracer::color::Color;
use ray_tracer::intersection::IntersectionList;
use ray_tracer::ppm_printer::PpmPrinter;
use ray_tracer::ray::Ray;
use ray_tracer::sphere::Sphere;
use ray_tracer::tuple::Tuple;

fn main() {
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 300.0;
    let pixel_size = wall_size / canvas_pixels;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels as usize, canvas_pixels as usize);
    let point_color = Color::new(1.0, 0.0, 0.0);
    let mut shape = Sphere::new();
    shape.transform = shape
        .transform
        .shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0)
        .scale(0.5, 1.0, 1.0);

    for y in 0..canvas_pixels as usize {
        let world_y = half - pixel_size * y as f64;

        for x in 0..canvas_pixels as usize {
            let world_x = -half + pixel_size * x as f64;
            let position = Tuple::point(world_x as f64, world_y as f64, wall_z);
            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let mut xs = IntersectionList::new(&shape.intersect(&r));
            if xs.hit().is_some() {
                canvas.write_pixel(x as usize, y as usize, point_color);
            }
        }
    }

    PpmPrinter::dump_to_file(&canvas, "sphere.ppm").unwrap();
}
