use ray_tracer::intersection::{Intersection, IntersectionList};
use ray_tracer::ray::Ray;
use ray_tracer::sphere::Sphere;
use ray_tracer::tuple::Tuple;

use ray_tracer::float_near_equal;

#[test]
fn init() {
    let s = Sphere::new();
    let i = Intersection::new(3.5, &s);
    assert!(float_near_equal(i.t, 3.5));
    assert_eq!(i.object, &s);
}

#[test]
fn intersect_sets_object() {
    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = s.intersect(&r);
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].object, &s);
    assert_eq!(xs[1].object, &s);
}

#[test]
fn sphere_intersection_center() {
    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = s.intersect(&r);
    assert_eq!(xs.len(), 2);
    assert!(float_near_equal(xs[0].t, 4.0));
    assert!(float_near_equal(xs[1].t, 6.0));
}

#[test]
fn sphere_intersection_tangent() {
    let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = s.intersect(&r);
    assert_eq!(xs.len(), 2);
    assert!(float_near_equal(xs[0].t, 5.0));
    assert!(float_near_equal(xs[1].t, 5.0));
}

#[test]
fn sphere_intersection_miss() {
    let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = s.intersect(&r);
    assert_eq!(xs.len(), 0);
}

#[test]
fn sphere_intersection_inside() {
    let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = s.intersect(&r);
    assert_eq!(xs.len(), 2);
    assert!(float_near_equal(xs[0].t, -1.0));
    assert!(float_near_equal(xs[1].t, 1.0));
}

#[test]
fn sphere_intersection_behind() {
    let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = s.intersect(&r);
    assert_eq!(xs.len(), 2);
    assert!(float_near_equal(xs[0].t, -6.0));
    assert!(float_near_equal(xs[1].t, -4.0));
}

#[test]
fn hit_all_positive() {
    let s = Sphere::new();
    let i1 = Intersection::new(1.0, &s);
    let i2 = Intersection::new(2.0, &s);
    let list = [&i1, &i2];
    let mut xs = IntersectionList::new(&list);
    let i = xs.hit().unwrap();
    assert_eq!(i, &i1);
}

#[test]
fn hit_some_negative() {
    let s = Sphere::new();
    let i1 = Intersection::new(-1.0, &s);
    let i2 = Intersection::new(1.0, &s);
    let list = [&i1, &i2];
    let mut xs = IntersectionList::new(&list);
    let i = xs.hit().unwrap();
    assert_eq!(i, &i2);
}

#[test]
fn hit_all_negative() {
    let s = Sphere::new();
    let i1 = Intersection::new(-1.0, &s);
    let i2 = Intersection::new(-2.0, &s);
    let list = [&i1, &i2];
    let mut xs = IntersectionList::new(&list);
    let i = xs.hit();
    assert_eq!(i, None);
}

#[test]
fn hit_gets_first() {
    let s = Sphere::new();
    let i1 = Intersection::new(5.0, &s);
    let i2 = Intersection::new(7.0, &s);
    let i3 = Intersection::new(-3.0, &s);
    let i4 = Intersection::new(2.0, &s);
    let list = [&i1, &i2, &i3, &i4];
    let mut xs = IntersectionList::new(&list);
    let i = xs.hit().unwrap();
    assert_eq!(i, &i4);
}

// #[test]
// fn aggregate_intersections() {
//     let s = Sphere::new();
//     let ix = vec![Intersection::new(1.0, &s), Intersection::new(2.0, &s)];
//     let xs = Intersection::intersections(&ix);
//     assert_eq!(xs.len(), 2);
//     assert!(float_near_equal(xs[0].t, 1.0));
//     assert!((float_near_equal(xs[1].t, 2.0)))
// }
