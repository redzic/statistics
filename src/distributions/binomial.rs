//! Implementation of the [Binomial](https://en.wikipedia.org/wiki/Binomial_distribution) distribution

use crate::stats::traits::*;
use std::convert::TryInto;

#[derive(Debug)]
pub struct Binomial {
    n: u64,
    p: f64,
}

impl Binomial {
    pub fn new(n: u64, p: f64) -> Self {
        Binomial { n, p }
    }
}

/// P(X = k)
impl PMF<u64> for Binomial {
    fn pmf(&self, k: u64) -> f64 {
        self.n.choose(k) as f64
            * self.p.powi(k.try_into().unwrap())
            * (1.0 - self.p).powf((self.n - k as u64) as f64)
    }
}

/// P(X < k)
impl CDF<u64> for Binomial {
    fn cdf(&self, k: u64) -> f64 {
        (0..k).map(|i| self.pmf(i)).sum::<f64>()
    }
}

impl Variance<f64> for Binomial {
    fn variance(&self) -> f64 {
        self.n as f64 * self.p * (1.0 - self.p)
    }
}

impl StdDev<f64> for Binomial {
    fn stdev(&self) -> f64 {
        self.variance().sqrt()
    }
}

impl Expected<f64> for Binomial {
    fn E(&self) -> f64 {
        self.n as f64 * self.p
    }
}

impl Mode<f64> for Binomial {
    // TODO add tests
    fn mode(&self) -> f64 {
        let condition = (self.n + 1) as f64 * self.p;

        if condition == 0.0 || (condition.floor() != condition) {
            return condition.floor();
        } else if condition.floor() == condition && condition as u64 <= self.n {
            // TODO is also condition - 1
            return condition;
        } else {
            return self.n as f64;
        }
    }
}
