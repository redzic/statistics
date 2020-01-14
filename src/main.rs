use rand::Rng;
mod distribution;
mod stats;
use crate::distribution::*;
use crate::stats::*;

fn main() {
    let mut rng = rand::thread_rng();

    let data = (0..10)
        .map(|_| rng.gen_range(0.0, 340.0))
        .collect::<Vec<f64>>();

    let dist = Normal::new(20.0, 3.46);

    println!("{}", dist.cdf(16.0));
}
