use ::statistics::distributions::*;
use ::statistics::functions::*;
use ::statistics::stats::*;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::Rng;

fn normal_cdf(x: f64) {
    Normal::new(0.0, 1.0).cdf(x);
}

fn normal_pdf(x: f64) {
    Normal::new(0.0, 1.0).pdf(x);
}

fn random_data(x: u32) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    (0..x).map(|_| rng.gen_range(-10.0, 10.0)).collect()
}

fn bench_stats(c: &mut Criterion) {
    let mut group = c.benchmark_group("stats");

    for i in [100, 1000, 5000, 15000, 150000].iter() {
        group.bench_with_input(BenchmarkId::new("stats", i), i, |b, i| {
            b.iter(|| random_data(*i).median())
        });
    }

    group.finish();
}

fn bench_erf(c: &mut Criterion) {
    let mut group = c.benchmark_group("error function");

    for i in [-1.5f64, 0f64, 1.5f64].iter() {
        group.bench_with_input(BenchmarkId::new("erf", i), i, |b, i| {
            b.iter(|| (*i).erf())
        });
    }

    group.finish();
}

fn bench_t(c: &mut Criterion) {
    let mut group = c.benchmark_group("T distribution");

    let dist = T::new(1);
    for i in [0.0, 0.5].iter() {
        group.bench_with_input(BenchmarkId::new("t_inv_cdf", i), i, |b, i| {
            b.iter(|| dist.ppf(*i))
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

criterion_group!(benches, bench_stats);
criterion_main!(benches);
