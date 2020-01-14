use rand::Rng;
mod distribution;
mod stats;
use crate::distribution::*;
use crate::stats::*;

fn main() {
    println!("{}", inverse_erf(0.3));
}
