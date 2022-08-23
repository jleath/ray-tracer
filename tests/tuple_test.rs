use ray_tracer::tuple::*;
use ray_tracer::*;

#[test]
fn tuple_init() {
    let point = Tuple::new(4.3, -4.2, 3.1, 1.0);
    assert!(float_near_equal(point.x, 4.3));
    assert!(float_near_equal(point.y, -4.2));
    assert!(float_near_equal(point.z, 3.1));
    assert!(float_near_equal(point.w, 1.0));
    assert!(point.is_point());
    assert!(!point.is_vector());

    let vector = Tuple::new(4.3, -4.2, 3.1, 0.0);
    assert!(vector.is_vector());
    assert!(!vector.is_point());
}

#[test]
fn point_init() {
    let point = Tuple::point(4.0, -4.0, 3.0);
    assert_eq!(point, Tuple::new(4.0, -4.0, 3.0, 1.0));
}

#[test]
fn vector_init() {
    let vector = Tuple::vector(4.0, -4.0, 3.0);
    assert_eq!(vector, Tuple::new(4.0, -4.0, 3.0, 0.0));
}

#[test]
fn addition() {
    // add vector to point produces point
    let mut a1 = Tuple::point(3.0, -2.0, 5.0);
    let a2 = Tuple::vector(-2.0, 3.0, 1.0);
    assert_eq!(a1 + a2, Tuple::point(1.0, 1.0, 6.0));

    // AddAssign trait
    a1 += a2;
    assert_eq!(a1, Tuple::point(1.0, 1.0, 6.0));
}

#[test]
fn subtraction() {
    // subtract point from point produces vector
    let mut a1 = Tuple::point(3.0, 2.0, 1.0);
    let mut a2 = Tuple::point(5.0, 6.0, 7.0);
    assert_eq!(a1 - a2, Tuple::vector(-2.0, -4.0, -6.0));

    // SubAssign trait
    a1 -= a2;
    assert_eq!(a1, Tuple::vector(-2.0, -4.0, -6.0));

    // subtract vector from point produces point
    a1 = Tuple::point(3.0, 2.0, 1.0);
    a2 = Tuple::vector(5.0, 6.0, 7.0);
    assert_eq!(a1 - a2, Tuple::point(-2.0, -4.0, -6.0));

    // subtract vector from vector produces vector
    a1 = Tuple::vector(3.0, 2.0, 1.0);
    a2 = Tuple::vector(5.0, 6.0, 7.0);
    assert_eq!(a1 - a2, Tuple::vector(-2.0, -4.0, -6.0));

    // subtracting from zero vector
    let zero = Tuple::vector(0.0, 0.0, 0.0);
    let v = Tuple::vector(1.0, -2.0, 3.0);
    assert_eq!(zero - v, Tuple::vector(-1.0, 2.0, -3.0));
}

#[test]
fn negation() {
    let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(-a, Tuple::new(-1.0, 2.0, -3.0, 4.0));
}

#[test]
fn scalar_mult() {
    let mut a = Tuple::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(a * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));

    a *= 3.5;
    assert_eq!(a, Tuple::new(3.5, -7.0, 10.5, -14.0));

    // mul by fraction
    a = Tuple::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(a * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0));
}

#[test]
fn scalar_div() {
    let mut a = Tuple::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(a / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));

    a /= 2.0;
    assert_eq!(a, Tuple::new(0.5, -1.0, 1.5, -2.0));
}

#[test]
fn magnitude() {
    let mut v = Tuple::vector(1.0, 0.0, 0.0);
    assert!(float_near_equal(v.magnitude(), 1.0));

    v = Tuple::vector(0.0, 1.0, 0.0);
    assert!(float_near_equal(v.magnitude(), 1.0));

    v = Tuple::vector(0.0, 0.0, 1.0);
    assert!(float_near_equal(v.magnitude(), 1.0));

    v = Tuple::vector(1.0, 2.0, 3.0);
    assert!(float_near_equal(v.magnitude(), (14.0_f64).sqrt()));

    v = Tuple::vector(-1.0, -2.0, -3.0);
    assert!(float_near_equal(v.magnitude(), (14.0_f64).sqrt()));
}

#[test]
fn normalize() {
    let mut v = Tuple::vector(4.0, 0.0, 0.0);
    assert_eq!(v.normalize(), Tuple::vector(1.0, 0.0, 0.0));

    v = Tuple::vector(1.0, 2.0, 3.0);
    assert_eq!(
        v.normalize(),
        Tuple::vector(
            1.0 / 14_f64.sqrt(),
            2.0 / 14_f64.sqrt(),
            3.0 / 14_f64.sqrt()
        )
    );

    assert!(float_near_equal(v.normalize().magnitude(), 1.0));
}

#[test]
fn dot_product() {
    let a = Tuple::vector(1.0, 2.0, 3.0);
    let b = Tuple::vector(2.0, 3.0, 4.0);
    assert!(float_near_equal(a.dot_product(&b), 20.0));
}

#[test]
fn cross_product() {
    let a = Tuple::vector(1.0, 2.0, 3.0);
    let b = Tuple::vector(2.0, 3.0, 4.0);
    assert_eq!(a.cross_product(&b), Tuple::vector(-1.0, 2.0, -1.0));
    assert_eq!(b.cross_product(&a), Tuple::vector(1.0, -2.0, 1.0));
}

#[test]
fn reflection() {
    let v = Tuple::vector(1.0, -1.0, 0.0);
    let n = Tuple::vector(0.0, 1.0, 0.0);
    let r = v.reflect(&n);
    assert_eq!(r, Tuple::vector(1.0, 1.0, 0.0));

    let v2 = Tuple::vector(0.0, -1.0, 0.0);
    let n2 = Tuple::vector(2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0, 0.0);
    let r2 = v2.reflect(&n2);
    assert_eq!(r2, Tuple::vector(1.0, 0.0, 0.0));
}
