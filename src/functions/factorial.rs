use super::gamma::*;

pub trait Factorial<T> {
    /// Falling factorial
    fn ffactorial(&self, n: f64) -> T;

    /// Rising factorial
    fn rfactorial(&self, n: f64) -> T;

    // Factorial
    fn factorial(&self) -> T;
}

impl Factorial<f64> for f64 {
    /// Falling factoral
    fn ffactorial(&self, n: f64) -> f64 {
        // TODO check for negative integer
        (self + 1.0).gamma() / (self - n + 1.0).gamma()
    }

    /// Rising factoral
    fn rfactorial(&self, n: f64) -> f64 {
        (self + n).gamma() / self.gamma()
    }

    /// Factorial
    fn factorial(&self) -> f64 {
        (self - 1.0).gamma()
    }
}
