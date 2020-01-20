use super::traits::*;
use rayon::prelude::*;

impl Mean for [f64] {
    fn mean(&self) -> f64 {
        self.iter().sum::<f64>() / self.len() as f64
    }
}

impl GeometricMean for [f64] {
    fn geometric_mean(&self) -> f64 {
        self.iter().product::<f64>().powf(1f64 / self.len() as f64)
    }
}

impl Median for [f64] {
    fn median(&self) -> f64 {
        let n = self.len();
        let mut copy = self.to_owned();

        copy.par_sort_unstable_by(|x, y| x.partial_cmp(y).unwrap());

        match n % 2 {
            0 => (copy[n / 2 - 1] + copy[n / 2]) / 2.0,
            _ => copy[n / 2],
        }
    }
}

impl Variance for [f64] {
    fn variance(&self) -> f64 {
        let mean = self.mean();
        self.iter().map(|i| (i - mean).powi(2)).sum::<f64>() / (self.len() as f64 - 1.0)
    }

    fn variance_with_mean(&self, mean: f64) -> f64 {
        self.iter().map(|i| (i - mean).powi(2)).sum::<f64>() / (self.len() as f64 - 1.0)
    }
}

impl StdDev for [f64] {
    fn stdev(&self) -> f64 {
        self.variance().sqrt()
    }

    fn stdev_with_mean(&self, mean: f64) -> f64 {
        self.variance_with_mean(mean).sqrt()
    }
}

impl Min for [f64] {
    fn min(&self) -> f64 {
        let mut min;

        unsafe {
            min = *self.get_unchecked(0);

            if self.len() == 1 {
                return min;
            }

            for i in 1..self.len() {
                if *self.get_unchecked(i) < min {
                    min = *self.get_unchecked(i);
                }
            }
        }

        min
    }
}

impl Max for [f64] {
    fn max(&self) -> f64 {
        let mut max;

        unsafe {
            max = *self.get_unchecked(0);

            if self.len() == 1 {
                return max;
            }

            for i in 1..self.len() {
                if *self.get_unchecked(i) > max {
                    max = *self.get_unchecked(i);
                }
            }
        }

        max
    }
}
