pub trait Mean<T> {
    fn mean(&self) -> T;
}

pub trait GeometricMean<T> {
    fn geometric_mean(&self) -> T;
}

pub trait Median<T> {
    fn median(&self) -> T;
}

pub trait Mode<T> {
    fn mode(&self) -> T;
}

pub trait StdDev<T> {
    fn stdev(&self) -> T;
    fn stdev_with_mean(&self, mean: f64) -> f64;
}

pub trait Variance<T> {
    fn variance(&self) -> T;
    fn variance_with_mean(&self, mean: f64) -> T;
}

pub trait Min<T> {
    fn min(&self) -> T;
}

pub trait Max<T> {
    fn max(&self) -> T;
}

pub trait BinomCoeff {
    // TODO make generic
    fn choose(&self, k: u64) -> u64;
}

pub trait Cdf {
    fn cdf(&self, x: f64) -> f64;
}

pub trait Pdf<T> {
    fn pdf(&self, x: T) -> f64;
}

pub trait InvCdf {
    fn inv_cdf(&self, p: f64) -> f64;
}
