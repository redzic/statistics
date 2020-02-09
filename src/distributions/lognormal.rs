//! Implementation of the [Log-normal](https://en.wikipedia.org/wiki/Log-normal_distribution) distribution

use crate::functions::erf::*;
use crate::stats::traits::*;

#[derive(Debug, Copy, Clone)]
pub struct LogNormal {
    mu: f64,
    sigma: f64,
}

impl LogNormal {
    pub fn new(mu: f64, sigma: f64) -> Self {
        if sigma <= 0.0 {
            panic!("σ must be positive")
        }

        LogNormal { mu, sigma }
    }
}

impl PDF<f64> for LogNormal {
    fn pdf(&self, x: f64) -> f64 {
        (-(x.ln() - self.mu).powi(2) / (2.0 * self.sigma.powi(2))).exp()
            / (x * self.sigma * (2.0 * std::f64::consts::PI).sqrt())
    }
}

impl CDF<f64> for LogNormal {
    fn cdf(&self, x: f64) -> f64 {
        0.5 + 0.5
            * ((x.ln() - self.mu) / (std::f64::consts::SQRT_2 * self.sigma)).erf()
    }
}

// TODO implement PPF

impl Mean<f64> for LogNormal {
    fn mean(&self) -> f64 {
        (self.mu + self.sigma.powi(2) / 2.0).exp()
    }
}

impl Median<f64> for LogNormal {
    fn median(&self) -> f64 {
        self.mu.exp()
    }
}

impl Mode<f64> for LogNormal {
    fn mode(&self) -> f64 {
        (self.mu - self.sigma.powi(2)).exp()
    }
}

impl Variance<f64> for LogNormal {
    fn variance(&self) -> f64 {
        ((self.sigma.powi(2)).exp() - 1.0)
            * (2.0 * self.mu + self.sigma.powi(2)).exp()
    }

    fn stdev(&self) -> f64 {
        self.variance().sqrt()
    }
}

impl Skewness<f64> for LogNormal {
    fn skewness(&self) -> f64 {
        (self.sigma.powi(2).exp() + 2.0) * (self.sigma.powi(2).exp() - 1.0).sqrt()
    }
}
