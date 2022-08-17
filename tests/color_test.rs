use ray_tracer::color::*;
use ray_tracer::*;

#[test]
fn color_init() {
    let c = Color::new(-0.5, 0.4, 1.7);
    assert!(float_near_equal(c.red, -0.5));
    assert!(float_near_equal(c.green, 0.4));
    assert!(float_near_equal(c.blue, 1.7));
}

#[test]
fn addition() {
    let mut c1 = Color::new(0.9, 0.6, 0.75);
    let c2 = Color::new(0.7, 0.1, 0.25);
    assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));

    c1 += c2;
    assert_eq!(c1, Color::new(1.6, 0.7, 1.0));
}

#[test]
fn subtraction() {
    let mut c1 = Color::new(0.9, 0.6, 0.75);
    let c2 = Color::new(0.7, 0.1, 0.25);
    assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));

    c1 -= c2;
    assert_eq!(c1, Color::new(0.2, 0.5, 0.5));
}

#[test]
fn multiplication() {
    let mut c1 = Color::new(0.2, 0.3, 0.4);
    assert_eq!(c1 * 2.0, Color::new(0.4, 0.6, 0.8));

    c1 *= 2.0;
    assert_eq!(c1, Color::new(0.4, 0.6, 0.8));

    c1 = Color::new(1.0, 0.2, 0.4);
    let c2 = Color::new(0.9, 1.0, 0.1);

    assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
    c1 *= c2;
    assert_eq!(c1, Color::new(0.9, 0.2, 0.04));
}
