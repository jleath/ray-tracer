use std::f64::consts::PI;

use ray_tracer::transform::*;
use ray_tracer::tuple::*;

#[test]
fn multiply_by_translation() {
    let p = Tuple::point(-3.0, 4.0, 5.0);
    let mut t = Transform::new();
    t = t.translate(5.0, -3.0, 2.0);
    assert_eq!(t.transform(&p), Tuple::point(2.0, 1.0, 7.0));
    t = t.inverse();
    assert_eq!(t.transform(&p), Tuple::point(-8.0, 7.0, 3.0));
}

#[test]
fn translate_vector() {
    let v = Tuple::vector(-3.0, 4.0, 5.0);
    assert_eq!(Transform::new().translate(5.0, -3.0, 2.).transform(&v), v);
}

#[test]
fn scale_point() {
    let p = Tuple::point(-4.0, 6.0, 8.0);
    assert_eq!(
        Transform::new().scale(2.0, 3.0, 4.0).transform(&p),
        Tuple::point(-8.0, 18.0, 32.0)
    );
}

#[test]
fn scale_vector() {
    let v = Tuple::vector(-4.0, 6.0, 8.0);
    assert_eq!(
        Transform::new().scale(2.0, 3.0, 4.0).transform(&v),
        Tuple::vector(-8.0, 18.0, 32.0)
    );
}

#[test]
fn scale_inverse() {
    let v = Tuple::vector(-4.0, 6.0, 8.0);
    assert_eq!(
        Transform::new()
            .scale(2.0, 3.0, 4.0)
            .inverse()
            .transform(&v),
        Tuple::vector(-2.0, 2.0, 2.0)
    );
}

#[test]
fn reflection() {
    let p = Tuple::point(2.0, 3.0, 4.0);
    assert_eq!(
        Transform::new().scale(-1.0, 1.0, 1.0).transform(&p),
        Tuple::point(-2.0, 3.0, 4.0)
    );
}

#[test]
fn rotate_x() {
    let p = Tuple::point(0.0, 1.0, 0.0);
    assert_eq!(
        Transform::new().rotate_x(PI / 4.0).transform(&p),
        Tuple::point(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0)
    );
    assert_eq!(
        Transform::new().rotate_x(PI / 2.0).transform(&p),
        Tuple::point(0.0, 0.0, 1.0)
    );
}

#[test]
fn inverse_rotate_x() {
    let p = Tuple::point(0.0, 1.0, 0.0);
    assert_eq!(
        Transform::new().rotate_x(PI / 4.0).inverse().transform(&p),
        Tuple::point(0.0, 2_f64.sqrt() / 2.0, -(2_f64.sqrt() / 2.0))
    );
}

#[test]
fn rotate_y() {
    let p = Tuple::point(0.0, 0.0, 1.0);
    assert_eq!(
        Transform::new().rotate_y(PI / 4.0).transform(&p),
        Tuple::point(2_f64.sqrt() / 2.0, 0.0, 2_f64.sqrt() / 2.0)
    );
    assert_eq!(
        Transform::new().rotate_y(PI / 2.0).transform(&p),
        Tuple::point(1.0, 0.0, 0.0)
    );
}

#[test]
fn rotate_z() {
    let p = Tuple::point(0.0, 1.0, 0.0);
    assert_eq!(
        Transform::new().rotate_z(PI / 4.0).transform(&p),
        Tuple::point(-(2_f64.sqrt() / 2.0), 2_f64.sqrt() / 2.0, 0.0)
    );
    assert_eq!(
        Transform::new().rotate_z(PI / 2.0).transform(&p),
        Tuple::point(-1.0, 0.0, 0.0)
    );
}

#[test]
fn shear() {
    let p = Tuple::point(2.0, 3.0, 4.0);

    assert_eq!(
        Transform::new()
            .shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0)
            .transform(&p),
        Tuple::point(5.0, 3.0, 4.0),
    );

    assert_eq!(
        Transform::new()
            .shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0)
            .transform(&p),
        Tuple::point(6.0, 3.0, 4.0)
    );

    assert_eq!(
        Transform::new()
            .shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0)
            .transform(&p),
        Tuple::point(2.0, 5.0, 4.0)
    );

    assert_eq!(
        Transform::new()
            .shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0)
            .transform(&p),
        Tuple::point(2.0, 7.0, 4.0)
    );

    assert_eq!(
        Transform::new()
            .shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0)
            .transform(&p),
        Tuple::point(2.0, 3.0, 6.0)
    );

    assert_eq!(
        Transform::new()
            .shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0)
            .transform(&p),
        Tuple::point(2.0, 3.0, 7.0)
    );
}

#[test]
fn transforms_in_sequence() {
    let p = Tuple::point(1.0, 0.0, 1.0);
    let p2 = Transform::new().rotate_x(PI / 2.0).transform(&p);
    assert_eq!(p2, Tuple::point(1.0, -1.0, 0.0));
    let p3 = Transform::new().scale(5.0, 5.0, 5.0).transform(&p2);
    assert_eq!(p3, Tuple::point(5.0, -5.0, 3.061616997868383e-16));
    let p4 = Transform::new().translate(10.0, 5.0, 7.0).transform(&p3);
    assert_eq!(p4, Tuple::point(15.0, 0.0, 7.0));
}

#[test]
fn builder() {
    let p = Tuple::point(1.0, 0.0, 1.0);
    assert_eq!(
        Transform::new()
            .rotate_x(PI / 2.0)
            .scale(5.0, 5.0, 5.0)
            .translate(10.0, 5.0, 7.0)
            .transform(&p),
        Tuple::point(15.0, 0.0, 7.0)
    );
}
