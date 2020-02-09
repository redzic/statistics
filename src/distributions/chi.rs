//! Implementation of the [Chi](https://en.wikipedia.org/wiki/Chi_distribution) distribution

use crate::functions::gamma::*;
use crate::stats::traits::*;

/// Chi distribution with `k` degrees of freedom
#[derive(Debug, Copy, Clone)]
pub struct Chi {
    k: f64,
}

impl Chi {
    pub fn new(k: f64) -> Self {
        if k <= 0.0 {
            panic!("degrees of freedom must be positive");
        }

        Chi { k }
    }
}

impl PDF<f64> for Chi {
    fn pdf(&self, x: f64) -> f64 {
        if x < 0.0 {
            return 0.0;
        }

        (x.powf(self.k - 1.0) * (-0.5 * x.powi(2)).exp())
            / (2f64.powf(0.5 * self.k - 1.0) * (0.5 * self.k).gamma())
    }
}

// impl CDF<f64> for Chi {
//     fn cdf(&self, x: f64) -> f64 {
//         0.0
//     }
// }

impl Mean<f64> for Chi {
    fn mean(&self) -> f64 {
        std::f64::consts::SQRT_2
            * ((0.5 * (self.k + 1.0)).gamma() / (0.5 * self.k).gamma())
    }
}

impl Median<f64> for Chi {
    fn median(&self) -> f64 {
        (self.k * (1.0 - (2.0 / (9.0 * self.k))).powi(3)).sqrt()
    }
}

impl Mode<f64> for Chi {
    fn mode(&self) -> f64 {
        if self.k >= 1.0 {
            return (self.k - 1.0).sqrt();
        }

        // TODO test this?
        std::f64::NAN
    }
}

impl Variance<f64> for Chi {
    fn variance(&self) -> f64 {
        self.k - self.mean().powi(2)
    }

    fn stdev(&self) -> f64 {
        self.variance().sqrt()
    }
}

impl Skewness<f64> for Chi {
    fn skewness(&self) -> f64 {
        let sigma = self.stdev();

        (self.mean() / sigma.powi(3)) * (1.0 - 2.0 * sigma.powi(2))
    }
}

impl ExcessKurtosis<f64> for Chi {
    fn ex_kurtosis(&self) -> f64 {
        let sigma = self.stdev();

        (2.0 / sigma.powi(2))
            * (1.0 - self.mean() * sigma * self.skewness() - sigma.powi(2))
    }
}

// TODO: implement polygamma function for this to work
// impl Entropy<f64> for Chi {
//     fn entropy(&self) -> f64 {
//         ((0.5 * self.k).gamma()).ln()
//             + 0.5 * (self.k - 2f64.ln() - (self.k - 1.0))
//     }
// }
