use std::f64::consts::{FRAC_1_SQRT_2, PI};

use ray_tracer::float_near_equal;
use ray_tracer::material::Material;
use ray_tracer::ray::Ray;
use ray_tracer::shape::Shape;
use ray_tracer::transform::Transform;
use ray_tracer::tuple::Tuple;

#[test]
fn init() {
    let s = Shape::sphere();
    assert_eq!(*s.transform(), Transform::new());
}

#[test]
fn intersect_scaled_sphere() {
    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let mut s = Shape::sphere();
    s.set_id(0);
    s.scale(2.0, 2.0, 2.0);
    let xs = s.intersect(&r).unwrap();
    assert_eq!(xs.len(), 2);
    assert!(float_near_equal(xs[0].t, 3.0));
    assert!(float_near_equal(xs[1].t, 7.0));
}

#[test]
fn intersect_translated_sphere() {
    let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    let mut s = Shape::sphere();
    s.translate(5.0, 0.0, 0.0);
    let xs = s.intersect(&r);
    assert!(xs.is_none());
}

#[test]
fn normal_at() {
    let s = Shape::sphere();
    let mut n = s.normal_at(Tuple::point(1.0, 0.0, 0.0));
    assert_eq!(n, Tuple::vector(1.0, 0.0, 0.0));

    n = s.normal_at(Tuple::point(0.0, 1.0, 0.0));
    assert_eq!(n, Tuple::vector(0.0, 1.0, 0.0));

    n = s.normal_at(Tuple::point(0.0, 0.0, 1.0));
    assert_eq!(n, Tuple::vector(0.0, 0.0, 1.0));

    n = s.normal_at(Tuple::point(
        3_f64.sqrt() / 3.0,
        3_f64.sqrt() / 3.0,
        3_f64.sqrt() / 3.0,
    ));
    assert_eq!(
        n,
        Tuple::vector(3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0)
    );
}

#[test]
fn normals_are_normalized() {
    let s = Shape::sphere();
    let n = s.normal_at(Tuple::point(
        3_f64.sqrt() / 3.0,
        3_f64.sqrt() / 3.0,
        3_f64.sqrt() / 3.0,
    ));

    assert_eq!(n, n.normalize());
}

#[test]
fn normal_after_transform() {
    let mut s = Shape::sphere();
    s.translate(0.0, 1.0, 0.0);
    let mut n = s.normal_at(Tuple::point(0.0, FRAC_1_SQRT_2 + 1.0, -FRAC_1_SQRT_2));
    assert_eq!(n, Tuple::vector(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));

    s = Shape::sphere();
    s.rotate_z(PI / 5.0);
    s.scale(1.0, 0.5, 1.0);

    n = s.normal_at(Tuple::point(0.0, 2_f64.sqrt() / 2.0, -(2_f64.sqrt()) / 2.0));
    assert_eq!(
        n,
        Tuple::vector(0.0, 0.9701425001453319, -0.24253562503633294)
    );
}

#[test]
fn sphere_material() {
    let mut s = Shape::sphere();
    let m = s.material();
    assert_eq!(*m, Material::new());

    let mut new_material = Material::new();
    new_material.ambient = 1.0;
    s.set_ambient(1.0);
    assert_eq!(*s.material(), new_material);
}
