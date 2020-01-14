use crate::stats::*;
use rand::Rng;

#[derive(Debug)]
pub struct Normal {
    mean: f64,
    stdev: f64,
}

impl Normal {
    // TODO create traits to avoid having to rewrite each method thing
    // TODO allow addition, subtraction, multiplication, and division of Normal
    // distributions
    pub fn new(mean: f64, stdev: f64) -> Self {
        Normal { mean, stdev }
    }

    pub fn from(data: &[f64]) -> Self {
        // TODO change functions so it doesn't keep recalculating the mean
        // aka provide optional mean parameter for the mean that you've already
        // calculated

        Normal {
            mean: mean(&data),
            stdev: stdev(&data),
        }
    }
    pub fn cdf(&self, x: f64) -> f64 {
        0.5 * (1.0 + erf((x - self.mean) / (self.stdev * std::f64::consts::SQRT_2)))
    }

    pub fn mean(&self) -> f64 {
        self.mean
    }

    pub fn median(&self) -> f64 {
        self.mean
    }

    pub fn mode(&self) -> f64 {
        self.mean
    }

    pub fn stdev(&self) -> f64 {
        self.stdev
    }

    pub fn variance(&self) -> f64 {
        self.stdev.powi(2)
    }

    // pub fn sample(&self, rng: &rand::Rng) -> f64 {
    //     2.0
    // }
}
