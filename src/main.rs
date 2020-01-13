use rand::Rng;
mod distribution;
mod stats;
use crate::distribution::*;
use crate::stats::*;
use std::time::Instant;

fn main() {
    let mut rng = rand::thread_rng();

    // let data = (0..100000)
    //     .map(|_| rng.gen_range(0.0, 340.0))
    //     .collect::<Vec<f64>>();

    let dist = Normal::new(0.0, 1.0);

    let start = Instant::now();
    // println!("Mean: {}", mean(&data));
    // println!("Median: {}", median(&data));
    // println!("σ²: {}", variance(&data));
    // println!("σ: {}", stdev(&data));

    println!("{}", dist.cdf(0.0));

    let duration = start.elapsed();

    println!("{:?} elapsed", duration);
}
