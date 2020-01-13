use crate::stats::*;

#[derive(Debug)]
pub struct Normal {
    mean: f64,
    stdev: f64,
}

impl Normal {
    pub fn new(mean: f64, stdev: f64) -> Self {
        Normal { mean, stdev }
    }

    pub fn cdf(&self, x: f64) -> f64 {
        0.5 * (1.0 + erf((x - self.mean) / (self.stdev * std::f64::consts::SQRT_2)))
    }
}
