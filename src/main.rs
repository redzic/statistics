use rand::Rng;
mod distribution;
mod stats;
use crate::distribution::*;
use crate::stats::*;

fn main() {
    // println!("{}", inverse_erf(0.25));
    println!("{}", Normal::new(0.0, 1.0).cdf(0.2));
}
