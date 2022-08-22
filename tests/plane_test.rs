use ray_tracer::float_near_equal;
use ray_tracer::ray::Ray;
use ray_tracer::shape::Shape;
use ray_tracer::tuple::Tuple;

#[test]
fn constant_normal() {
    let p = Shape::plane();
    let n = p.normal_at(Tuple::point(0.0, 0.0, 0.0));
    let n2 = p.normal_at(Tuple::point(10.0, 0.0, -10.0));
    let n3 = p.normal_at(Tuple::point(-5.0, 0.0, 150.0));

    assert_eq!(n, Tuple::vector(0.0, 1.0, 0.0));
    assert_eq!(n2, Tuple::vector(0.0, 1.0, 0.0));
    assert_eq!(n3, Tuple::vector(0.0, 1.0, 0.0));
}

#[test]
fn intersect() {
    let mut p = Shape::plane();
    p.set_id(0);
    let r = Ray::new(Tuple::point(0.0, 10.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
    let xs = p.intersect(&r).unwrap();
    assert_eq!(xs.len(), 0);

    let r2 = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
    let xs2 = p.intersect(&r2).unwrap();
    assert_eq!(xs2.len(), 0);

    let r3 = Ray::new(Tuple::point(0.0, 1.0, 0.0), Tuple::vector(0.0, -1.0, 0.0));
    let xs3 = p.intersect(&r3).unwrap();
    assert_eq!(xs3.len(), 1);
    assert!(float_near_equal(xs3[0].t, 1.0));

    let r4 = Ray::new(Tuple::point(0.0, -1.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));
    let xs4 = p.intersect(&r4).unwrap();
    assert_eq!(xs4.len(), 1);
    assert!(float_near_equal(xs4[0].t, 1.0));
}
