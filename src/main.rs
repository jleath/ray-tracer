use ray_tracer::camera::Camera;
use ray_tracer::color::Color;
use ray_tracer::pattern::Pattern;
use ray_tracer::point_light::PointLight;
use ray_tracer::ppm_printer::PpmPrinter;
use ray_tracer::shape::Shape;
use ray_tracer::transform::Transform;
use ray_tracer::tuple::Tuple;
use ray_tracer::world::World;
use std::f64::consts::PI;

fn main() {
    let mut floor = Shape::plane();
    // let mut checkers = Pattern::checkered(Color::new(0.0, 0.0, 0.0), Color::new(1.0, 1.0, 1.0));
    // checkers.scale(1.1, 1.1, 1.1);
    floor.set_reflective(0.8);
    // checkers.translate(1.0, 0.0, 0.0);
    // floor.set_pattern(&checkers);
    floor.scale(1.0, 0.01, 1.0);
    floor.set_color(Color::new(0.45, 0.45, 0.45));
    floor.set_specular(0.0);

    let mut middle = Shape::sphere();
    middle.translate(-0.5, 1.0, 0.5);
    middle.set_color(Color::new(0.75, 0.00, 0.00));
    middle.set_diffuse(0.7);
    middle.set_specular(0.85);

    let mut right = Shape::sphere();
    right.scale(0.5, 0.5, 0.5);
    right.translate(1.5, 1.5, -0.5);
    right.set_color(Color::new(0.5, 1.0, 0.1));
    right.set_diffuse(0.7);
    right.set_specular(0.3);

    let mut left = Shape::sphere();
    left.scale(0.33, 0.33, 0.33);
    left.translate(-1.5, 0.33, -0.75);
    left.set_color(Color::new(1.0, 0.8, 0.1));
    left.set_specular(0.7);
    left.set_diffuse(0.3);

    let mut world = World::new();
    world.add_light(PointLight::new(
        Tuple::point(-10.0, 10.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    ));

    world.add_object(floor);
    world.add_object(middle);
    world.add_object(right);
    world.add_object(left);

    let mut camera = Camera::new(500.0, 250.0, PI / 3.0);
    camera.transform = Transform::view_transform(
        &Tuple::point(0.0, 1.5, -9.0),
        &Tuple::point(0.0, 1.0, 0.0),
        &Tuple::vector(0.0, 1.0, 0.0),
    );

    let image = camera.render(&world);

    PpmPrinter::dump_to_file(&image, "darker.ppm").unwrap();
}
