use rand::Rng;
mod distribution;
mod stats;
use crate::distribution::*;
use crate::stats::*;

fn main() {
    println!("{}", Normal::new(0.0, 1.0).inv_cdf(0.95));
    println!("{}", Normal::new(0.0, 1.0).cdf(0.55));
    println!("{}", Normal::new(0.0, 1.0).mean());
    println!("{}", Normal::new(0.0, 1.0).median());
    println!("{}", Normal::new(0.0, 1.0).mode());
    println!("{}", Normal::new(0.0, 1.0).stdev());
    println!("{}", Normal::new(0.0, 1.0).variance());
}
