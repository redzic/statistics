use rand::Rng;
mod measure;
use crate::measure::measure::*;
use std::time::Instant;

fn main() {
    let mut rng = rand::thread_rng();

    let data = (0..100000)
        .map(|_| rng.gen_range(0.0, 340.0))
        .collect::<Vec<f64>>();

    let start = Instant::now();
    println!("Mean: {}", mean(&data));
    println!("Median: {}", median(&data));
    println!("σ²: {}", variance(&data));
    println!("σ: {}", stdev(&data));

    println!("{}", 1.0 - normal_cdf(6.0, 0.0, 1.0));
    let duration = start.elapsed();

    println!("{:?} elapsed", duration);
}
