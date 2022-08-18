use ray_tracer::float_near_equal;
use ray_tracer::matrix::*;
use ray_tracer::tuple::*;

#[test]
fn init() {
    let m = Matrix::new(&vec![
        vec![1.0, 2.0, 3.0, 4.0],
        vec![5.5, 6.5, 7.5, 8.5],
        vec![9.0, 10.0, 11.0, 12.0],
        vec![13.5, 14.5, 15.5, 16.5],
    ]);
    assert!(float_near_equal(1.0, m.get(0, 0)));
    assert!(float_near_equal(4.0, m.get(0, 3)));
    assert!(float_near_equal(5.5, m.get(1, 0)));
    assert!(float_near_equal(7.5, m.get(1, 2)));
    assert!(float_near_equal(11.0, m.get(2, 2)));
    assert!(float_near_equal(13.5, m.get(3, 0)));
    assert!(float_near_equal(15.5, m.get(3, 2)));

    let m2 = Matrix::new(&vec![vec![-3.0, 5.0], vec![1.0, -2.0]]);
    assert!(float_near_equal(-3.0, m2.get(0, 0)));
    assert!(float_near_equal(5.0, m2.get(0, 1)));
    assert!(float_near_equal(1.0, m2.get(1, 0)));
    assert!(float_near_equal(-2.0, m2.get(1, 1)));

    let m3 = Matrix::new(&vec![
        vec![-3.0, 5.0, 0.0],
        vec![1.0, -2.0, -7.0],
        vec![0.0, 1.0, 1.0],
    ]);

    assert!(float_near_equal(-3.0, m3.get(0, 0)));
    assert!(float_near_equal(-2.0, m3.get(1, 1)));
    assert!(float_near_equal(1.0, m3.get(2, 2)));
}

#[test]
fn equality() {
    let m1 = Matrix::new(&vec![
        vec![1.0, 2.0, 3.0, 4.0],
        vec![5.5, 6.5, 7.5, 8.5],
        vec![9.0, 10.0, 11.0, 12.0],
        vec![13.5, 14.5, 15.5, 16.5],
    ]);
    let m2 = Matrix::new(&vec![
        vec![1.0, 2.0, 3.0, 4.0],
        vec![5.5, 6.5, 7.5, 8.5],
        vec![9.0, 10.0, 11.0, 12.0],
        vec![13.5, 14.5, 15.5, 16.5],
    ]);

    assert_eq!(m1, m2);

    let m3 = Matrix::new(&vec![
        vec![1.0, 2.0, 3.0, 4.0],
        vec![5.5, 6.5, 7.5, 8.5],
        vec![9.0, 9.0, 11.0008, 12.0],
        vec![13.5, 14.5, 15.5, 16.5],
    ]);
    assert_ne!(m1, m3);
}

#[test]
fn multiply4x4() {
    let m1 = Matrix::new(&vec![
        vec![1.0, 2.0, 3.0, 4.0],
        vec![5.0, 6.0, 7.0, 8.0],
        vec![9.0, 8.0, 7.0, 6.0],
        vec![5.0, 4.0, 3.0, 2.0],
    ]);

    let m2 = Matrix::new(&vec![
        vec![-2.0, 1.0, 2.0, 3.0],
        vec![3.0, 2.0, 1.0, -1.0],
        vec![4.0, 3.0, 6.0, 5.0],
        vec![1.0, 2.0, 7.0, 8.0],
    ]);

    let expected = Matrix::new(&vec![
        vec![20.0, 22.0, 50.0, 48.0],
        vec![44.0, 54.0, 114.0, 108.0],
        vec![40.0, 58.0, 110.0, 102.0],
        vec![16.0, 26.0, 46.0, 42.0],
    ]);

    assert_eq!(m1 * m2, expected);
}

#[test]
fn multiply_tuple() {
    let m = Matrix::new(&vec![
        vec![1.0, 2.0, 3.0, 4.0],
        vec![2.0, 4.0, 4.0, 2.0],
        vec![8.0, 6.0, 4.0, 1.0],
        vec![0.0, 0.0, 0.0, 1.0],
    ]);

    let t = Tuple::new(1.0, 2.0, 3.0, 1.0);

    assert_eq!(m * t, Tuple::new(18.0, 24.0, 33.0, 1.0));
}

#[test]
fn multiply_by_identity() {
    let m = Matrix::new(&vec![
        vec![1.0, 2.0, 3.0, 4.0],
        vec![2.0, 4.0, 4.0, 2.0],
        vec![8.0, 6.0, 4.0, 1.0],
        vec![0.0, 0.0, 0.0, 1.0],
    ]);

    let expected = Matrix::new(&vec![
        vec![1.0, 2.0, 3.0, 4.0],
        vec![2.0, 4.0, 4.0, 2.0],
        vec![8.0, 6.0, 4.0, 1.0],
        vec![0.0, 0.0, 0.0, 1.0],
    ]);

    assert_eq!(m * Matrix::identity_matrix(), expected);
}
