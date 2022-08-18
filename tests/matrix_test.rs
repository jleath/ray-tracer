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

    assert_eq!(m1.matrix_multiply(&m2), expected);
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

    assert_eq!(m.tuple_multiply(&t), Tuple::new(18.0, 24.0, 33.0, 1.0));
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

    assert_eq!(m.matrix_multiply(&Matrix::identity_matrix()), expected);
}

#[test]
fn transpose() {
    let m = Matrix::new(&vec![
        vec![0.0, 9.0, 3.0, 0.0],
        vec![9.0, 8.0, 0.0, 8.0],
        vec![1.0, 8.0, 5.0, 3.0],
        vec![0.0, 0.0, 5.0, 8.0],
    ]);

    let expected = Matrix::new(&vec![
        vec![0.0, 9.0, 1.0, 0.0],
        vec![9.0, 8.0, 8.0, 0.0],
        vec![3.0, 0.0, 5.0, 5.0],
        vec![0.0, 8.0, 3.0, 8.0],
    ]);

    assert_eq!(m.transpose(), expected);

    assert_eq!(
        Matrix::identity_matrix().transpose(),
        Matrix::identity_matrix()
    );
}

#[test]
fn determinant2x2() {
    let m = Matrix::new(&vec![vec![1.0, 5.0], vec![-3.0, 2.0]]);
    assert!(float_near_equal(m.determinant(), 17.0));
}

#[test]
fn determinant3x3() {
    let m = Matrix::new(&vec![
        vec![1.0, 2.0, 6.0],
        vec![-5.0, 8.0, -4.0],
        vec![2.0, 6.0, 4.0],
    ]);
    assert!(float_near_equal(m.cofactor(0, 0), 56.0));
    assert!(float_near_equal(m.cofactor(0, 1), 12.0));
    assert!(float_near_equal(m.cofactor(0, 2), -46.0));
    assert!(float_near_equal(m.determinant(), -196.0));
}

#[test]
fn determinant4x4() {
    let m = Matrix::new(&vec![
        vec![-2.0, -8.0, 3.0, 5.0],
        vec![-3.0, 1.0, 7.0, 3.0],
        vec![1.0, 2.0, -9.0, 6.0],
        vec![-6.0, 7.0, 7.0, -9.0],
    ]);

    assert!(float_near_equal(m.cofactor(0, 0), 690.0));
    assert!(float_near_equal(m.cofactor(0, 1), 447.0));
    assert!(float_near_equal(m.cofactor(0, 2), 210.0));
    assert!(float_near_equal(m.cofactor(0, 3), 51.0));
    assert!(float_near_equal(m.determinant(), -4071.0));
}

#[test]
fn submatrix() {
    let m3 = Matrix::new(&vec![
        vec![1.0, 5.0, 0.0],
        vec![-3.0, 2.0, 7.0],
        vec![0.0, 6.0, -3.0],
    ]);

    assert_eq!(
        m3.submatrix(0, 2),
        Matrix::new(&vec![vec![-3.0, 2.0], vec![0.0, 6.0]])
    );

    let m4 = Matrix::new(&vec![
        vec![-6.0, 1.0, 1.0, 6.0],
        vec![-8.0, 5.0, 8.0, 6.0],
        vec![-1.0, 0.0, 8.0, 2.0],
        vec![-7.0, 1.0, -1.0, 1.0],
    ]);

    assert_eq!(
        m4.submatrix(2, 1),
        Matrix::new(&vec![
            vec![-6.0, 1.0, 6.0],
            vec![-8.0, 8.0, 6.0],
            vec![-7.0, -1.0, 1.0]
        ]),
    )
}

#[test]
fn minor() {
    let m = Matrix::new(&vec![
        vec![3.0, 5.0, 0.0],
        vec![2.0, -1.0, -7.0],
        vec![6.0, -1.0, 5.0],
    ]);

    let b = m.submatrix(1, 0);
    assert!(float_near_equal(b.determinant(), 25.0));
    assert!(float_near_equal(m.minor(1, 0), 25.0));
}

#[test]
fn cofactor() {
    let m = Matrix::new(&vec![
        vec![3.0, 5.0, 0.0],
        vec![2.0, -1.0, -7.0],
        vec![6.0, -1.0, 5.0],
    ]);

    assert!(float_near_equal(m.minor(0, 0), -12.0));
    assert!(float_near_equal(m.cofactor(0, 0), -12.0));
    assert!(float_near_equal(m.minor(1, 0), 25.0));
    assert!(float_near_equal(m.cofactor(1, 0), -25.0));
}

#[test]
fn is_invertible() {
    let m = Matrix::new(&vec![
        vec![6.0, 4.0, 4.0, 4.0],
        vec![5.0, 5.0, 7.0, 6.0],
        vec![4.0, -9.0, 3.0, -7.0],
        vec![9.0, 1.0, 7.0, -6.0],
    ]);

    assert!(float_near_equal(m.determinant(), -2120.0));
    assert!(m.is_invertible());

    let m2 = Matrix::new(&vec![
        vec![-4.0, 2.0, -2.0, -3.0],
        vec![9.0, 6.0, 2.0, 6.0],
        vec![0.0, -5.0, 1.0, -5.0],
        vec![0.0, 0.0, 0.0, 0.0],
    ]);

    assert!(float_near_equal(m2.determinant(), 0.0));
    assert!(!m2.is_invertible());
}

#[test]
fn inverse() {
    let a = Matrix::new(&vec![
        vec![-5.0, 2.0, 6.0, -8.0],
        vec![1.0, -5.0, 1.0, 8.0],
        vec![7.0, 7.0, -6.0, -7.0],
        vec![1.0, -3.0, 7.0, 4.0],
    ]);

    let b = a.inverse();
    assert!(float_near_equal(a.determinant(), 532.0));
    assert!(float_near_equal(a.cofactor(2, 3), -160.0));
    assert!(float_near_equal(b.get(2, 3), 105.0 / 532.0));
    assert!(float_near_equal(a.cofactor(3, 2), 105.0));
    assert!(float_near_equal(b.get(2, 3), 105.0 / 532.0));

    let expected = Matrix::new(&vec![
        vec![
            0.21804511278195488,
            0.45112781954887216,
            0.24060150375939848,
            -0.045112781954887216,
        ],
        vec![
            -0.8082706766917294,
            -1.4567669172932332,
            -0.44360902255639095,
            0.5206766917293233,
        ],
        vec![
            -0.07894736842105263,
            -0.2236842105263158,
            -0.05263157894736842,
            0.19736842105263158,
        ],
        vec![
            -0.5225563909774437,
            -0.8139097744360902,
            -0.3007518796992481,
            0.30639097744360905,
        ],
    ]);

    assert_eq!(b, expected);

    let a2 = Matrix::new(&vec![
        vec![8.0, -5.0, 9.0, 2.0],
        vec![7.0, 5.0, 6.0, 1.0],
        vec![-6.0, 0.0, 9.0, 6.0],
        vec![-3.0, 0.0, -9.0, -4.0],
    ]);

    let expected2 = Matrix::new(&vec![
        vec![
            -0.15384615384615385,
            -0.15384615384615385,
            -0.28205128205128205,
            -0.5384615384615384,
        ],
        vec![
            -0.07692307692307693,
            0.12307692307692308,
            0.02564102564102564,
            0.03076923076923077,
        ],
        vec![
            0.358974358974359,
            0.358974358974359,
            0.4358974358974359,
            0.9230769230769231,
        ],
        vec![
            -0.6923076923076923,
            -0.6923076923076923,
            -0.7692307692307693,
            -1.9230769230769231,
        ],
    ]);

    assert_eq!(a2.inverse(), expected2);

    let a3 = Matrix::new(&vec![
        vec![9.0, 3.0, 0.0, 9.0],
        vec![-5.0, -2.0, -6.0, -3.0],
        vec![-4.0, 9.0, 6.0, 4.0],
        vec![-7.0, 6.0, 6.0, 2.0],
    ]);

    let expected3 = Matrix::new(&vec![
        vec![
            -0.040740740740740744,
            -0.07777777777777778,
            0.14444444444444443,
            -0.2222222222222222,
        ],
        vec![
            -0.07777777777777778,
            0.03333333333333333,
            0.36666666666666664,
            -0.3333333333333333,
        ],
        vec![
            -0.029012345679012345,
            -0.14629629629629629,
            -0.10925925925925926,
            0.12962962962962962,
        ],
        vec![
            0.17777777777777778,
            0.06666666666666667,
            -0.26666666666666666,
            0.3333333333333333,
        ],
    ]);

    assert_eq!(a3.inverse(), expected3);
}

#[test]
fn multiply_by_inverse() {
    let a = Matrix::new(&vec![
        vec![3.0, -9.0, 7.0, 3.0],
        vec![3.0, -8.0, 2.0, -9.0],
        vec![-4.0, 4.0, 4.0, 1.0],
        vec![-6.0, 5.0, -1.0, 1.0],
    ]);
    let b = Matrix::new(&vec![
        vec![8.0, 2.0, 2.0, 2.0],
        vec![3.0, -1.0, 7.0, 0.0],
        vec![7.0, 0.0, 5.0, 4.0],
        vec![6.0, -2.0, 0.0, 5.0],
    ]);

    let expected = Matrix::new(&vec![
        vec![3.0, -9.0, 7.0, 3.000000000000001],
        vec![
            2.999999999999999,
            -7.999999999999998,
            2.0000000000000036,
            -8.999999999999996,
        ],
        vec![
            -3.9999999999999996,
            3.9999999999999996,
            3.9999999999999982,
            0.9999999999999993,
        ],
        vec![-6.0, 5.0, -1.0, 0.9999999999999998],
    ]);

    let c = a.matrix_multiply(&b);
    assert_eq!(c.matrix_multiply(&b.inverse()), expected);
}
