use ray_tracer::float_near_equal;
use ray_tracer::ray::Ray;
use ray_tracer::sphere::Sphere;
use ray_tracer::transform::Transform;
use ray_tracer::tuple::Tuple;

#[test]
fn init() {
    let s = Sphere::new();
    assert_eq!(s.transform, Transform::new());
}

#[test]
fn intersect_scaled_sphere() {
    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let mut s = Sphere::new();

    s.transform = s.transform.scale(2.0, 2.0, 2.0);
    let xs = s.intersect(&r);
    assert_eq!(xs.len(), 2);
    assert!(float_near_equal(xs[0].t, 3.0));
    assert!(float_near_equal(xs[1].t, 7.0));
}

#[test]
fn intersect_translated_sphere() {
    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let mut s = Sphere::new();
    s.transform = s.transform.translate(5.0, 0.0, 0.0);
    let xs = s.intersect(&r);
    assert_eq!(xs.len(), 0);
}
