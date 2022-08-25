#![warn(clippy::pedantic)]

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
    let mut checkers = Pattern::checkered(Color::new(0.0, 0.0, 0.0), Color::new(1.0, 1.0, 1.0));
    checkers.scale(1.1, 1.1, 1.1);
    floor.translate(0.0, 0.0, 5.0);
    floor.set_pattern(&checkers);
    floor.scale(1.0, 0.01, 1.0);
    floor.set_specular(0.0);
    floor.set_reflective(0.8);

    let mut big_ball = Shape::glass_sphere();
    big_ball.set_color(Color::new(0.3, 0.3, 0.3));
    big_ball.set_transparency(0.9);
    big_ball.set_reflective(0.9);
    big_ball.translate(-0.5, 1.0, 0.5);
    big_ball.set_diffuse(0.5);
    big_ball.set_ambient(0.5);
    big_ball.set_specular(0.3);

    let mut small_ball = Shape::sphere();
    small_ball.scale(0.5, 0.5, 0.5);
    small_ball.translate(1.5, 0.5, -0.5);
    small_ball.set_transparency(1.0);
    small_ball.set_refractive_index(1.00029);

    let mut world = World::new();
    world.add_light(PointLight::new(
        Tuple::point(-10.0, 10.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    ));

    world.add_object(floor);
    world.add_object(big_ball);
    world.add_object(small_ball);

    let mut camera = Camera::new(800.0, 400.0, PI / 3.0);
    camera.transform = Transform::view_transform(
        &Tuple::point(0.0, 1.5, -8.0),
        &Tuple::point(0.0, 1.0, 0.0),
        &Tuple::vector(0.0, 1.0, 0.0),
    );

    let image = camera.render(&world);

    PpmPrinter::dump_to_file(&image, "darker.ppm").unwrap();
}
