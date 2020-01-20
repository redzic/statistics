pub trait Mean {
    fn mean(&self) -> f64;
}

pub trait GeometricMean {
    fn geometric_mean(&self) -> f64;
}

pub trait Median {
    fn median(&self) -> f64;
}

pub trait Mode {
    fn mode(&self) -> f64;
}

pub trait StdDev {
    fn stdev(&self) -> f64;
    fn stdev_with_mean(&self, mean: f64) -> f64;
}

pub trait Variance {
    fn variance(&self) -> f64;
    fn variance_with_mean(&self, mean: f64) -> f64;
}

pub trait Min {
    fn min(&self) -> f64;
}

pub trait Max {
    fn max(&self) -> f64;
}

pub trait Cdf {
    fn cdf(&self, x: f64) -> f64;
}

pub trait Pdf {
    fn pdf(&self, x: f64) -> f64;
}

pub trait InvCdf {
    fn inv_cdf(&self, p: f64) -> f64;
}
