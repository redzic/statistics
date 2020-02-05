//! Implementation of the [Weibull](https://en.wikipedia.org/wiki/Weibull_distribution) distribution

use crate::functions::gamma::*;
use crate::stats::traits::*;

#[derive(Debug, Copy, Clone)]
/// Weibull distribution where lambda is the scale and k is the shape.
pub struct Weibull {
    // scale
    lambda: f64,
    // shape
    k: f64,
}

impl Weibull {
    pub fn new(lambda: f64, k: f64) -> Self {
        if lambda > 0.0 && k > 0.0 {
            Self { lambda, k }
        } else {
            panic!("λ and k must be positive")
        }
    }
}

impl PDF<f64> for Weibull {
    fn pdf(&self, x: f64) -> f64 {
        if x >= 0.0 {
            (self.k / self.lambda)
                * (x / self.lambda).powf(self.lambda - 1.0)
                * (-(x / self.lambda).powf(self.k)).exp()
        } else {
            0.0
        }
    }
}

impl CDF<f64> for Weibull {
    fn cdf(&self, x: f64) -> f64 {
        if x >= 0.0 {
            1.0 - (-(x / self.lambda).powf(self.k)).exp()
        } else {
            0.0
        }
    }
}

impl Mean<f64> for Weibull {
    fn mean(&self) -> f64 {
        self.lambda * (1.0 + 1.0 / self.k).gamma()
    }
}

impl Median<f64> for Weibull {
    fn median(&self) -> f64 {
        self.lambda * (2f64.ln()).powf(1.0 / self.k)
    }
}

impl Mode<f64> for Weibull {
    fn mode(&self) -> f64 {
        if self.k > 1.0 {
            self.lambda * ((self.k - 1.0) / self.k).powf(1.0 / self.k)
        } else {
            0.0
        }
    }
}

impl Variance<f64> for Weibull {
    fn variance(&self) -> f64 {
        self.lambda.powi(2)
            * ((1.0 + 2.0 / self.k).gamma()
                - (1.0 + 1.0 / self.k).gamma().powi(2))
    }
}
