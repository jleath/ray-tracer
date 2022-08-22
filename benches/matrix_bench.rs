#![allow(unused_must_use)]
use criterion::{criterion_group, criterion_main, Criterion};
use ray_tracer::matrix::Matrix;

fn transpose() {
    let m = Matrix::identity_matrix();
    m.transpose();
}

fn determinant() {
    let m = Matrix::identity_matrix();
    m.determinant();
}

fn submatrix() {
    let m = Matrix::identity_matrix();
    m.submatrix(1, 2);
}

fn inverse() {
    let m = Matrix::identity_matrix();
    m.inverse();
}

fn multiply() {
    let m1 = Matrix::identity_matrix();
    let m2 = Matrix::identity_matrix();
    m1.matrix_multiply(&m2);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("matrix_transpose", |b| b.iter(transpose));
    c.bench_function("matrix_determinant", |b| b.iter(determinant));
    c.bench_function("matrix_submatrix", |b| b.iter(submatrix));
    c.bench_function("matrix inverse", |b| b.iter(inverse));
    c.bench_function("matrix multiply", |b| b.iter(multiply));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
