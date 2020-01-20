use ::statistics::*;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn normal_cdf(x: f64) {
    let dist = Normal::new(0.0, 1.0);
    dist.cdf_lossy(x);
}

fn normal_pdf(x: f64) {
    let dist = Normal::new(0.0, 1.0);
    dist.pdf(x);
}

fn bench_norm_cdf(c: &mut Criterion) {
    let mut group = c.benchmark_group("Distrbutions");

    for i in [-1.0, -0.5, 0.0, 0.5, 1.0].iter() {
        group.bench_with_input(BenchmarkId::new("normal_cdf", i), i, |b, i| {
            b.iter(|| normal_cdf(*i))
        });
    }

    group.finish();
}

fn bench_norm_pdf(c: &mut Criterion) {
    let mut group = c.benchmark_group("Distrbutions");

    for i in [-1.0, 0.0, 1.0].iter() {
        group.bench_with_input(BenchmarkId::new("normal_pdf", i), i, |b, i| {
            b.iter(|| normal_pdf(*i))
        });
    }

    group.finish();
}

fn min_max() {
    let data = [
        0.0, 0.0, 2.0, -0.2, 5.0, -0.264, 61.9, -54.6, 72.9, 3.0, -0.2,
    ];

    data.max();
}

fn bench_min_max(c: &mut Criterion) {
    c.bench_function("min_max", |b| b.iter(|| min_max()));
}

criterion_group!(benches, bench_min_max);
criterion_main!(benches);
