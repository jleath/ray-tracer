#![allow(unused_must_use)]
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ray_tracer::canvas::Canvas;
use ray_tracer::color::Color;
use ray_tracer::ppm_printer::PpmPrinter;

fn ppm_printer(image: &Canvas) {
    PpmPrinter::pixel_data(image);
}

fn criterion_benchmark(c: &mut Criterion) {
    let width = 100;
    let height = 50;
    let mut canvas = Canvas::new(width, height);
    for y in 0..height {
        for x in 0..width {
            canvas.write_pixel(x, y, Color::new(0.8, 1.0, 6.0));
        }
    }

    c.bench_function("ppm_printer", |b| {
        b.iter(|| ppm_printer(black_box(&canvas)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
