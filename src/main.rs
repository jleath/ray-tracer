use ray_tracer::camera::Camera;
use ray_tracer::color::Color;
use ray_tracer::point_light::PointLight;
use ray_tracer::ppm_printer::PpmPrinter;
use ray_tracer::sphere::Sphere;
use ray_tracer::transform::Transform;
use ray_tracer::tuple::Tuple;
use ray_tracer::world::World;
use std::f64::consts::PI;

fn main() {
    let mut floor = Sphere::new();
    floor.scale(10.0, 0.01, 10.0);
    floor.set_color(Color::new(1.0, 0.9, 0.9));
    floor.set_specular(0.0);

    let mut left_wall = Sphere::new();
    left_wall.scale(10.0, 0.01, 10.0);
    left_wall.rotate_x(PI / 2.0);
    left_wall.rotate_y(-PI / 4.0);
    left_wall.translate(0.0, 0.0, 5.0);
    left_wall.material = floor.material;

    let mut right_wall = Sphere::new();
    right_wall.scale(10.0, 0.01, 10.0);
    right_wall.rotate_x(PI / 2.0);
    right_wall.rotate_y(PI / 4.0);
    right_wall.translate(0.0, 0.0, 5.0);
    right_wall.material = floor.material;

    let mut middle = Sphere::new();
    middle.translate(-0.5, 1.0, 0.5);
    middle.set_color(Color::new(0.1, 1.0, 0.5));
    middle.set_diffuse(0.7);
    middle.set_specular(0.3);

    let mut right = Sphere::new();
    right.scale(0.5, 0.5, 0.5);
    right.translate(1.5, 0.5, -0.5);
    right.set_color(Color::new(0.5, 1.0, 0.1));
    right.set_diffuse(0.7);
    right.set_specular(0.3);

    let mut left = Sphere::new();
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
    world.add_object(left_wall);
    world.add_object(right_wall);
    world.add_object(middle);
    world.add_object(right);
    world.add_object(left);

    let mut camera = Camera::new(1500.0, 750.0, PI / 3.0);
    camera.transform = Transform::view_transform(
        &Tuple::point(0.0, 1.5, -5.0),
        &Tuple::point(0.0, 1.0, 0.0),
        &Tuple::vector(0.0, 1.0, 0.0),
    );

    let image = camera.render(&world);

    PpmPrinter::dump_to_file(&image, "smallworld.ppm").unwrap();
}
