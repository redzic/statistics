use ::statistics::*;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn normal_cdf(x: f64) {
    Normal::new(0.0, 1.0).cdf(x);
}

fn normal_pdf(x: f64) {
    Normal::new(0.0, 1.0).pdf(x);
}

fn bench_erf(c: &mut Criterion) {
    let mut group = c.benchmark_group("error function");

    for i in [-1.5, 0.0, 1.5].iter() {
        group.bench_with_input(BenchmarkId::new("erf", i), i, |b, i| {
            b.iter(|| (*i).erf())
        });
    }

    group.finish();
}

fn bench_norm_cdf(c: &mut Criterion) {
    let mut group = c.benchmark_group("normal CDF");

    for i in [-1.0, -0.5, 0.0, 0.5, 1.0].iter() {
        group.bench_with_input(BenchmarkId::new("normal_cdf", i), i, |b, i| {
            b.iter(|| normal_cdf(*i))
        });
    }

    group.finish();
}

fn bench_norm_pdf(c: &mut Criterion) {
    let mut group = c.benchmark_group("normal PDF");

    for i in [-1.0, 0.0, 1.0].iter() {
        group.bench_with_input(BenchmarkId::new("normal_pdf", i), i, |b, i| {
            b.iter(|| normal_pdf(*i))
        });
    }

    group.finish();
}

fn binom_pdf() {
    let dist = Binomial::new(200, 0.03);
    dist.pmf(53);
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

fn bench_binom(c: &mut Criterion) {
    c.bench_function("binom", |b| b.iter(|| binom_pdf()));
}

// TODO benchmark gamma function

criterion_group!(benches, bench_erf);
criterion_main!(benches);
