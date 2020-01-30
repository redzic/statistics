use crate::functions::beta::*;
use crate::functions::gamma::*;
use crate::stats::traits::*;

pub struct T {
    // degrees of freedom
    nu: f64,
}

impl T {
    pub fn new(nu: f64) -> Self {
        T { nu }
    }
}

impl PDF<f64> for T {
    fn pdf(&self, t: f64) -> f64 {
        ((0.5 * (self.nu + 1.0)).gamma()
            / ((self.nu * std::f64::consts::PI).sqrt() * (0.5 * self.nu).gamma()))
            * (1.0 + t.powi(2) / self.nu).powf(-0.5 * (self.nu + 1.0))
    }
}

impl CDF<f64> for T {
    fn cdf(&self, t: f64) -> f64 {
        let x_t = self.nu / (t * t + self.nu);
        1.0 - 0.5 * beta_inc(0.5 * self.nu, 0.5, x_t)
    }
}
