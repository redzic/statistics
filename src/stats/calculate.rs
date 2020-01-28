use super::traits::*;
use rayon::prelude::*;

impl Mean<f64> for [f64] {
    fn mean(&self) -> f64 {
        self.iter().sum::<f64>() / self.len() as f64
    }
}

impl HarmonicMean<f64> for [f64] {
    fn harmonic_mean(&self) -> f64 {
        self.len() as f64 / self.iter().map(|i| 1f64 / i).sum::<f64>()
    }
}

impl GeometricMean<f64> for [f64] {
    fn geometric_mean(&self) -> f64 {
        self.iter().product::<f64>().powf(1f64 / self.len() as f64)
    }
}

impl Median<f64> for [f64] {
    fn median(&self) -> f64 {
        let n = self.len();
        let mut data = self.to_owned();

        data.par_sort_unstable_by(|x, y| x.partial_cmp(y).unwrap());

        match n % 2 {
            0 => (data[n / 2 - 1] + data[n / 2]) / 2.0,
            _ => data[n / 2],
        }
    }
}

impl Variance<f64> for [f64] {
    fn variance(&self) -> f64 {
        let mean = self.mean();
        self.iter().map(|i| (i - mean).powi(2)).sum::<f64>()
            / (self.len() as f64 - 1.0)
    }

    fn variance_with_mean(&self, mean: f64) -> f64 {
        self.iter().map(|i| (i - mean).powi(2)).sum::<f64>()
            / (self.len() as f64 - 1.0)
    }
}

impl StdDev<f64> for [f64] {
    fn stdev(&self) -> f64 {
        self.variance().sqrt()
    }

    fn stdev_with_mean(&self, mean: f64) -> f64 {
        self.variance_with_mean(mean).sqrt()
    }
}

impl Min<f64> for [f64] {
    // TODO add check for len 0
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

impl Max<f64> for [f64] {
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

impl BinomCoeff for u64 {
    fn choose(&self, mut k: u64) -> u64 {
        let mut b = 1u64;

        if k > self - k {
            k = self - k;
        }

        {
            let mut i = 0;
            while i < k {
                b *= self - i;
                b /= i + 1;
                i += 1;
            }
        }

        b
    }
}
