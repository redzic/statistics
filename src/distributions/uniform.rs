use crate::statistics::traits::*;

pub struct Uniform {
    min: f64,
    max: f64,
}

impl Min for Uniform {
    fn min(&self) -> f64 {
        self.min
    }
}

impl Max for Uniform {
    fn max(&self) -> f64 {
        self.max
    }
}

impl Mean for Uniform {
    fn mean(&self) -> f64 {
        (self.min + self.max) / 2.0
    }
}

impl Median for Uniform {
    fn median(&self) -> f64 {
        self.mean()
    }
}
