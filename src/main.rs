use rand::Rng;
mod distribution;
mod stats;
use crate::distribution::*;
use crate::stats::*;

fn main() {
    println!("{}", Normal::new(0.0, 1.0).inv_cdf(0.95));
}
