//! Implementation of the [Degenerate](https://en.wikipedia.org/wiki/Degenerate_distribution) distribution

use crate::stats::traits::*;

#[derive(Debug, Copy, Clone)]
pub struct Degenerate {
    k0: f64,
}

impl Degenerate {
    pub fn new(k0: f64) -> Self {
        Self { k0 }
    }
}

impl Mean<f64> for Degenerate {
    fn mean(&self) -> f64 {
        self.k0
    }
}

impl Mode<f64> for Degenerate {
    fn mode(&self) -> f64 {
        self.k0
    }
}

impl Median<f64> for Degenerate {
    fn median(&self) -> f64 {
        self.k0
    }
}

impl Variance<f64> for Degenerate {
    fn variance(&self) -> f64 {
        0.0
    }

    fn stdev(&self) -> f64 {
        0.0
    }
}

impl PMF<f64> for Degenerate {
    fn pmf(&self, x: f64) -> f64 {
        if x == self.k0 { 1.0 } else { 0.0 }
    }
}

impl CDF<f64> for Degenerate {
    fn cdf(&self, x: f64) -> f64 {
        if x >= self.k0 { 1.0 } else { 0.0 }
    }
}
