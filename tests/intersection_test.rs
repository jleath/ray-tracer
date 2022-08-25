use ray_tracer::intersection::{Intersection, IntersectionList};
use ray_tracer::ray::Ray;
use ray_tracer::shape::Shape;
use ray_tracer::tuple::Tuple;
use ray_tracer::world::World;

use ray_tracer::float_near_equal;
use ray_tracer::EPSILON;

#[test]
fn init() {
    let i = Intersection::new(3.5, 0);
    assert!(float_near_equal(i.t, 3.5));
}

#[test]
fn intersect_sets_object() {
    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let mut s = Shape::sphere();
    s.set_id(0);
    let xs = s.intersect(&r).unwrap();
    assert_eq!(xs.len(), 2);
    assert_eq!(xs[0].object_id, 0);
    assert_eq!(xs[1].object_id, 0);
}

#[test]
fn sphere_intersection_center() {
    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let mut s = Shape::sphere();
    s.set_id(0);
    let xs = s.intersect(&r).unwrap();
    assert_eq!(xs.len(), 2);
    assert!(float_near_equal(xs[0].t, 4.0));
    assert!(float_near_equal(xs[1].t, 6.0));
}

#[test]
fn sphere_intersection_tangent() {
    let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let mut s = Shape::sphere();
    s.set_id(0);
    let xs = s.intersect(&r).unwrap();
    assert_eq!(xs.len(), 2);
    assert!(float_near_equal(xs[0].t, 5.0));
    assert!(float_near_equal(xs[1].t, 5.0));
}

#[test]
fn sphere_intersection_miss() {
    let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let mut s = Shape::sphere();
    s.set_id(0);
    let xs = s.intersect(&r).unwrap();
    assert_eq!(xs.len(), 0);
}

#[test]
fn sphere_intersection_inside() {
    let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
    let mut s = Shape::sphere();
    s.set_id(0);
    let xs = s.intersect(&r).unwrap();
    assert_eq!(xs.len(), 2);
    assert!(float_near_equal(xs[0].t, -1.0));
    assert!(float_near_equal(xs[1].t, 1.0));
}

#[test]
fn sphere_intersection_behind() {
    let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
    let mut s = Shape::sphere();
    s.set_id(0);
    let xs = s.intersect(&r).unwrap();
    assert_eq!(xs.len(), 2);
    assert!(float_near_equal(xs[0].t, -6.0));
    assert!(float_near_equal(xs[1].t, -4.0));
}

#[test]
fn hit_all_positive() {
    let i1 = Intersection::new(1.0, 0);
    let i2 = Intersection::new(2.0, 0);
    let list = vec![i1, i2];
    let mut xs = IntersectionList::new(&list);
    let i = xs.hit().unwrap();
    assert_eq!(i, &i1);
}

#[test]
fn hit_some_negative() {
    let i1 = Intersection::new(-1.0, 0);
    let i2 = Intersection::new(1.0, 0);
    let list = vec![i1, i2];
    let mut xs = IntersectionList::new(&list);
    let i = xs.hit().unwrap();
    assert_eq!(i, &i2);
}

#[test]
fn hit_all_negative() {
    let i1 = Intersection::new(-1.0, 0);
    let i2 = Intersection::new(-2.0, 0);
    let list = vec![i1, i2];
    let mut xs = IntersectionList::new(&list);
    let i = xs.hit();
    assert_eq!(i, None);
}

#[test]
fn hit_gets_first() {
    let i1 = Intersection::new(5.0, 0);
    let i2 = Intersection::new(7.0, 0);
    let i3 = Intersection::new(-3.0, 0);
    let i4 = Intersection::new(2.0, 0);
    let list = vec![i1, i2, i3, i4];
    let mut xs = IntersectionList::new(&list);
    let i = xs.hit().unwrap();
    assert_eq!(i, &i4);
}

#[test]
fn prepare_comps() {
    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let s = Shape::sphere();
    let mut world = World::new();
    let object_id = world.add_object(s);
    let i = Intersection::new(4.0, object_id);
    let comps = i.prepare_computation(&r, &world);
    assert_eq!(comps.object_id, object_id);
    assert_eq!(comps.point, Tuple::point(0.0, 0.0, -1.0));
    assert_eq!(comps.eyev, Tuple::vector(0.0, 0.0, -1.0));
    assert_eq!(comps.normalv, Tuple::vector(0.0, 0.0, -1.0));
    assert!(!comps.inside);

    let r2 = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
    let i2 = Intersection::new(1.0, object_id);
    let comps2 = i2.prepare_computation(&r2, &world);
    assert_eq!(comps2.point, Tuple::point(0.0, 0.0, 1.0));
    assert_eq!(comps2.eyev, Tuple::vector(0.0, 0.0, -1.0));
    assert_eq!(comps2.normalv, Tuple::vector(0.0, 0.0, -1.0));
    assert!(comps2.inside);
}

#[test]
fn precompute_reflection() {
    let shape = Shape::plane();
    let mut world = World::new();
    let r = Ray::new(
        Tuple::point(0.0, 1.0, -1.0),
        Tuple::vector(0.0, -(2_f64.sqrt()) / 2.0, (2_f64.sqrt()) / 2.0),
    );
    world.add_object(shape);
    let i = Intersection::new(2_f64.sqrt(), 0);
    let comps = i.prepare_computation(&r, &world);
    assert_eq!(
        comps.reflectv,
        Tuple::vector(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0)
    );
}

#[test]
fn precompute_n1_and_n2() {
    let mut world = World::new();
    let mut a = Shape::glass_sphere();
    a.scale(2.0, 2.0, 2.0);
    a.set_refractive_index(1.5);
    let mut b = Shape::glass_sphere();
    b.translate(0.0, 0.0, -0.25);
    b.set_refractive_index(2.0);
    let mut c = Shape::glass_sphere();
    c.translate(0.0, 0.0, 0.25);
    c.set_refractive_index(2.5);

    world.add_object(a);
    world.add_object(b);
    world.add_object(c);

    let r = Ray::new(Tuple::point(0.0, 0.0, -4.0), Tuple::vector(0.0, 0.0, 1.0));
    let intersections = vec![
        Intersection::new(2.0, 0),
        Intersection::new(2.75, 1),
        Intersection::new(3.25, 2),
        Intersection::new(4.75, 1),
        Intersection::new(5.25, 2),
        Intersection::new(6.0, 0),
    ];
    let ix = IntersectionList::new(&intersections);

    let expect = [
        [1.0, 1.5],
        [1.5, 2.0],
        [2.0, 2.5],
        [2.5, 2.5],
        [2.5, 1.5],
        [1.5, 1.0],
    ];

    for (idx, _) in intersections.iter().enumerate() {
        let comps = ix.prepare_computation(idx, &r, &world);
        assert!(float_near_equal(comps.n1, expect[idx][0]));
        assert!(float_near_equal(comps.n2, expect[idx][1]));
    }
}

#[test]
fn hit_offsets_point() {
    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let mut shape = Shape::sphere();
    shape.translate(0.0, 0.0, 1.0);
    let mut world = World::new();
    let id = world.add_object(shape);
    let i = Intersection::new(5.0, id);
    let comps = i.prepare_computation(&r, &world);
    assert!(comps.over_point.z < EPSILON / 2.0);
    assert!(comps.point.z > comps.over_point.z);
}
