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

impl Pmf<u64> for Binomial {
    fn pmf(&self, k: u64) -> f64 {
        self.n.choose(k) as f64
            * self.p.powi(k.try_into().unwrap())
            * (1.0 - self.p).powf((self.n - k as u64) as f64)
    }
}
