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

criterion_group!(benches, bench_norm_pdf);
criterion_main!(benches);
