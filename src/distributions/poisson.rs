use crate::stats::traits::*;
use rug::{ops::Pow, Float, Integer};

#[derive(Debug, Copy, Clone)]
pub struct Poisson {
    lambda: f64,
}

impl Poisson {
    pub fn new(lambda: f64) -> Self {
        Poisson { lambda }
    }
}

impl Mean<f64> for Poisson {
    fn mean(&self) -> f64 {
        self.lambda
    }
}

impl Variance<f64> for Poisson {
    fn variance(&self) -> f64 {
        self.lambda
    }
}

impl StdDev<f64> for Poisson {
    fn stdev(&self) -> f64 {
        self.variance().sqrt()
    }
}

/// P(X = k)
impl PMF<u32> for Poisson {
    fn pmf(&self, k: u32) -> f64 {
        let _k = Integer::from(k);
        let _lambda = Float::with_val(53, self.lambda);
        // TODO change to not use arbitrary precision
        // and/or make this less ugly
        ((_lambda.pow(_k) * (-self.lambda).exp())
            / (Integer::from(Integer::factorial(k)).to_f64()))
        .to_f64()
    }
}

/// P(X < k)
// TODO maybe switch to other algorithm
impl CDF<u32> for Poisson {
    fn cdf(&self, k: u32) -> f64 {
        (0..k).map(|i| self.pmf(i)).sum::<f64>()
    }
}

impl Skewness<f64> for Poisson {
    fn skewness(&self) -> f64 {
        1.0 / self.lambda.sqrt()
    }
}
