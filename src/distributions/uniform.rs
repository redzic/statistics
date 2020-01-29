use crate::stats::traits::*;

#[derive(Debug, Copy, Clone)]
pub struct Uniform {
    min: f64,
    max: f64,
}

impl Uniform {
    pub fn new(min: f64, max: f64) -> Self {
        Uniform { min, max }
    }
}

impl Min<f64> for Uniform {
    fn min(&self) -> f64 {
        self.min
    }
}

impl Max<f64> for Uniform {
    fn max(&self) -> f64 {
        self.max
    }
}

impl Mean<f64> for Uniform {
    fn mean(&self) -> f64 {
        0.5 * (self.min + self.max)
    }
}

impl Median<f64> for Uniform {
    fn median(&self) -> f64 {
        0.5 * (self.min + self.max)
    }
}

impl Variance<f64> for Uniform {
    fn variance(&self) -> f64 {
        (self.max - self.min).powi(2) / 12.0
    }
}

impl StdDev<f64> for Uniform {
    fn stdev(&self) -> f64 {
        self.variance().sqrt()
    }
}

impl PDF<f64> for Uniform {
    fn pdf(&self, x: f64) -> f64 {
        if x >= self.min && x <= self.max {
            1.0 / (self.max - self.min)
        } else {
            0.0
        }
    }
}

impl CDF<f64> for Uniform {
    fn cdf(&self, x: f64) -> f64 {
        if x < self.min {
            0.0
        } else if x > self.max {
            1.0
        } else {
            (x - self.min) / (self.max - self.min)
        }
    }
}
