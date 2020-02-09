// use crate::functions::beta::*;
use crate::functions::gamma::*;
use crate::stats::traits::*;

/// Gamma distribution where alpha is the shape and beta is the rate
#[derive(Debug, Copy, Clone)]
pub struct Gamma {
    alpha: f64,
    beta: f64,
}

impl Gamma {
    pub fn new(alpha: f64, beta: f64) -> Self {
        if alpha <= 0.0 || beta <= 0.0 {
            panic!("α and β must be positive");
        }

        Gamma { alpha, beta }
    }
}

impl PDF<f64> for Gamma {
    fn pdf(&self, x: f64) -> f64 {
        (self.beta.powf(self.alpha) / gamma(self.alpha))
            * x.powf(self.alpha - 1.0)
            * (-self.beta * x).exp()
    }
}

// impl CDF<f64> for Gamma {
//     fn cdf(&self, x: f64) -> f64 {
//         lincgamma(self.alpha, self.beta * x) / self.alpha.gamma()
//     }
// }

impl Mean<f64> for Gamma {
    fn mean(&self) -> f64 {
        self.alpha / self.beta
    }
}

impl Mode<f64> for Gamma {
    fn mode(&self) -> f64 {
        (self.alpha - 1.0) / self.beta
    }
}

impl Variance<f64> for Gamma {
    fn variance(&self) -> f64 {
        self.alpha / self.beta.powi(2)
    }

    fn stdev(&self) -> f64 {
        self.variance().sqrt()
    }
}

impl Skewness<f64> for Gamma {
    fn skewness(&self) -> f64 {
        2.0 / self.alpha.sqrt()
    }
}
