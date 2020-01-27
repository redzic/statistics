use crate::stats::traits::*;

#[derive(Debug)]
pub struct Uniform {
    min: f64,
    max: f64,
}

impl Min<f64> for Uniform {
    fn min(&self) -> f64 {
        self.min
    }
}

impl Max<f64> for Uniform {
    fn max(&self) -> f64 {
        self.max
    }
}

impl Mean<f64> for Uniform {
    fn mean(&self) -> f64 {
        (self.min + self.max) / 2.0
    }
}

impl Median<f64> for Uniform {
    fn median(&self) -> f64 {
        self.mean()
    }
}
