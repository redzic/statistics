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

/// Binomial Coefficient
pub trait BinomCoeff {
    fn choose(&self, k: u64) -> u64;
}

/// Cumulative Distribution Function
pub trait CDF<T> {
    fn cdf(&self, x: T) -> f64;
}

/// Probability Density Function
pub trait PDF<T> {
    fn pdf(&self, x: T) -> f64;
}

/// Probability Mass Function
pub trait PMF<T> {
    fn pmf(&self, x: T) -> f64;
}

pub trait InverseCDF {
    fn inv_cdf(&self, p: f64) -> f64;
}
