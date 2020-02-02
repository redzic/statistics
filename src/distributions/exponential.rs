use crate::stats::traits::*;

#[derive(Debug, Copy, Clone)]
pub struct Exponential {
    // rate or inverse scale
    lambda: f64,
}

impl Exponential {
    pub fn new(lambda: f64) -> Self {
        if lambda > 0.0 {
            Self { lambda }
        } else {
            panic!("λ must be positive");
        }
    }
}

impl Mean<f64> for Exponential {
    fn mean(&self) -> f64 {
        1.0 / self.lambda
    }
}

impl Median<f64> for Exponential {
    fn median(&self) -> f64 {
        2f64.ln() / self.lambda
    }
}

impl Mode<f64> for Exponential {
    fn mode(&self) -> f64 {
        0.0
    }
}

impl PDF<f64> for Exponential {
    fn pdf(&self, x: f64) -> f64 {
        if x >= 0.0 {
            self.lambda * (-self.lambda * x).exp()
        } else {
            0.0
        }
    }
}

impl CDF<f64> for Exponential {
    fn cdf(&self, x: f64) -> f64 {
        1.0 - (-self.lambda * x).exp()
    }
}

impl PPF<f64> for Exponential {
    fn ppf(&self, p: f64) -> f64 {
        if 0.0 <= p && p < 1.0 {
            -(1.0 - p).ln() / self.lambda
        } else {
            std::f64::NAN
        }
    }
}

impl Variance<f64> for Exponential {
    fn variance(&self) -> f64 {
        1.0 / self.lambda.powi(2)
    }
}

impl Min<f64> for Exponential {
    fn min(&self) -> f64 {
        0.0
    }
}

impl Max<f64> for Exponential {
    fn max(&self) -> f64 {
        std::f64::INFINITY
    }
}

impl Skewness<f64> for Exponential {
    fn skewness(&self) -> f64 {
        2.0
    }
}
