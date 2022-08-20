use std::f64::consts::PI;

use ray_tracer::camera::Camera;
use ray_tracer::color::Color;
use ray_tracer::float_near_equal;
use ray_tracer::transform::Transform;
use ray_tracer::tuple::Tuple;
use ray_tracer::world::World;

#[test]
fn init() {
    let hsize = 160.0;
    let vsize = 120.0;
    let field_of_view = PI / 2.0;
    let c = Camera::new(hsize, vsize, field_of_view);
    assert!(float_near_equal(c.hsize, 160.0));
    assert!(float_near_equal(c.vsize, 120.0));
    assert!(float_near_equal(c.field_of_view, PI / 2.0));
    assert_eq!(c.transform, Transform::new());
}

#[test]
fn pixel_size() {
    let c = Camera::new(200.0, 125.0, PI / 2.0);
    assert!(float_near_equal(c.pixel_size, 0.01));

    let c2 = Camera::new(125.0, 200.0, PI / 2.0);
    assert!(float_near_equal(c2.pixel_size, 0.01));
}

#[test]
#[allow(clippy::approx_constant)]
fn ray_construction() {
    let mut c = Camera::new(201.0, 101.0, PI / 2.0);
    let through_center = c.ray_for_pixel(100.0, 50.0);
    assert_eq!(through_center.origin, Tuple::point(0.0, 0.0, 0.0));
    assert_eq!(through_center.direction, Tuple::vector(0.0, 0.0, -1.0));

    let through_corner = c.ray_for_pixel(0.0, 0.0);
    assert_eq!(through_corner.origin, Tuple::point(0.0, 0.0, 0.0));
    assert_eq!(
        through_corner.direction,
        Tuple::vector(0.6651864261194508, 0.3325932130597254, -0.6685123582500481)
    );

    c.transform = c.transform.translate(0.0, -2.0, 5.0);
    c.transform = c.transform.rotate_y(PI / 4.0);
    let after_transform = c.ray_for_pixel(100.0, 50.0);
    assert_eq!(after_transform.origin, Tuple::point(0.0, 2.0, -5.0));
    assert_eq!(
        after_transform.direction,
        Tuple::vector(0.7071067811865475, 0.0, -0.7071067811865478)
    )
}

#[test]
fn render() {
    let w = World::default_world();
    let mut c = Camera::new(11.0, 11.0, PI / 2.0);
    let from = Tuple::point(0.0, 0.0, -5.0);
    let to = Tuple::point(0.0, 0.0, 0.0);
    let up = Tuple::vector(0.0, 1.0, 0.0);
    c.transform = Transform::view_transform(&from, &to, &up);
    let image = c.render(&w);
    assert_eq!(
        image.pixel_at(5, 5),
        Color::new(
            0.38066119308103435,
            0.47582649135129296,
            0.28549589481077575
        )
    );
}
