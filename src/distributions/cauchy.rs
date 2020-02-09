//! Implementation of the [Cauchy](https://en.wikipedia.org/wiki/Cauchy_distribution) distribution

use crate::stats::traits::*;

/// Cauchy distribution with location `x0` and scale `gamma`
#[derive(Debug, Copy, Clone)]
pub struct Cauchy {
    x0: f64,
    gamma: f64,
}

impl Cauchy {
    pub fn new(x0: f64, gamma: f64) -> Self {
        if gamma <= 0.0 {
            panic!("γ must be positive");
        }

        Cauchy { x0, gamma }
    }
}

impl PDF<f64> for Cauchy {
    fn pdf(&self, x: f64) -> f64 {
        1.0 / (std::f64::consts::PI
            * self.gamma
            * (1.0 + ((x - self.x0) / self.gamma).powi(2)))
    }
}

impl CDF<f64> for Cauchy {
    fn cdf(&self, x: f64) -> f64 {
        (1.0 / std::f64::consts::PI) * ((x - self.x0) / self.gamma).atan() + 0.5
    }
}

impl PPF<f64> for Cauchy {
    fn ppf(&self, p: f64) -> f64 {
        self.x0 + self.gamma * (std::f64::consts::PI * (p - 0.5)).tan()
    }
}

impl Mean<f64> for Cauchy {
    fn mean(&self) -> f64 {
        std::f64::NAN
    }
}

impl Median<f64> for Cauchy {
    fn median(&self) -> f64 {
        self.x0
    }
}

impl Mode<f64> for Cauchy {
    fn mode(&self) -> f64 {
        self.x0
    }
}

impl Variance<f64> for Cauchy {
    fn variance(&self) -> f64 {
        std::f64::NAN
    }

    fn stdev(&self) -> f64 {
        std::f64::NAN
    }
}

impl Skewness<f64> for Cauchy {
    fn skewness(&self) -> f64 {
        std::f64::NAN
    }
}
