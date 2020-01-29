use crate::functions::erf::*;
use crate::stats::traits::*;
use std::cmp::PartialEq;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Normal {
    mean: f64,
    stdev: f64,
}

impl Normal {
    pub fn new(mean: f64, stdev: f64) -> Self {
        if stdev <= 0.0 {
            panic!("σ must be positive");
        }

        Normal { mean, stdev }
    }

    pub fn from(data: &[f64]) -> Self {
        let mean = data.mean();

        Normal {
            mean,
            stdev: data.stdev_with_mean(mean),
        }
    }
}

impl Add<Normal> for Normal {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            mean: self.mean + other.mean,
            stdev: (self.stdev.powi(2) + other.stdev.powi(2)).sqrt(),
        }
    }
}

impl Add<f64> for Normal {
    type Output = Self;

    fn add(self, scalar: f64) -> Self {
        Self {
            mean: self.mean + scalar,
            stdev: self.stdev,
        }
    }
}

impl Mul<f64> for Normal {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            mean: self.mean * scalar,
            stdev: self.stdev * scalar,
        }
    }
}

// Clippy is wrong in this case
#[allow(clippy::suspicious_arithmetic_impl)]
impl Sub<Normal> for Normal {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            mean: self.mean - other.mean,
            stdev: (self.stdev.powi(2) + other.stdev.powi(2)).sqrt(),
        }
    }
}

impl Sub<f64> for Normal {
    type Output = Self;

    fn sub(self, scalar: f64) -> Self {
        Self {
            mean: self.mean - scalar,
            stdev: self.stdev,
        }
    }
}

impl PartialEq for Normal {
    fn eq(&self, other: &Self) -> bool {
        self.mean == other.mean && self.stdev == other.stdev
    }
}

impl Mean<f64> for Normal {
    fn mean(&self) -> f64 {
        self.mean
    }
}

impl Median<f64> for Normal {
    fn median(&self) -> f64 {
        self.mean
    }
}

impl Mode<f64> for Normal {
    fn mode(&self) -> f64 {
        self.mean
    }
}

impl StdDev<f64> for Normal {
    fn stdev(&self) -> f64 {
        self.stdev
    }

    // TODO clean up with API somehow with optional arguments
    // (which I don't think are even possible)
    // This function literally makes no sense in this case
    // same thing for variance
    fn stdev_with_mean(&self, _mean: f64) -> f64 {
        self.stdev
    }
}

impl Variance<f64> for Normal {
    fn variance(&self) -> f64 {
        self.stdev.powi(2)
    }

    fn variance_with_mean(&self, _mean: f64) -> f64 {
        self.stdev
    }
}

impl CDF<f64> for Normal {
    fn cdf(&self, x: f64) -> f64 {
        0.5 * (1.0
            + ((x - self.mean) / (self.stdev * std::f64::consts::SQRT_2)).erf())
    }
}

impl InverseCDF for Normal {
    /// Compute the inverse cumulative distribution function for the given Normal
    /// distribution. Returns NaN if p < 0.0 or p > 1.0.
    fn inv_cdf(&self, p: f64) -> f64 {
        let q = p - 0.5;

        let num: f64;
        let den: f64;

        if q.abs() <= 0.425 {
            let r = 0.180_625 - q.powi(2);

            num = (((((((2.509_080_928_730_122_7e3 * r
                + 3.343_057_558_358_813e4)
                * r
                + 6.726_577_092_700_87e4)
                * r
                + 4.592_195_393_154_987e4)
                * r
                + 1.373_169_376_550_946e4)
                * r
                + 1.971_590_950_306_551_3e3)
                * r
                + 1.331_416_678_917_843_8e2)
                * r
                + 3.387_132_872_796_366_5e0)
                * q;

            den = ((((((5.226_495_278_852_854e3 * r
                + 2.872_908_573_572_194_3e4)
                * r
                + 3.930_789_580_009_271e4)
                * r
                + 2.121_379_430_158_659_7e4)
                * r
                + 5.394_196_021_424_751e3)
                * r
                + 6.871_870_074_920_579e2)
                * r
                + 4.231_333_070_160_091e1)
                * r
                + 1.0;

            let x = num / den;
            return self.mean + (x * self.stdev);
        }

        let mut r = if q <= 0.0 { p } else { 1.0 - p };
        r = (-r.ln()).sqrt();

        if r <= 5.0 {
            r -= 1.6;

            num = ((((((7.745_450_142_783_414e-4 * r
                + 2.272_384_498_926_918_4e-2)
                * r
                + 2.417_807_251_774_506e-1)
                * r
                + 1.270_458_252_452_368_4e0)
                * r
                + 3.647_848_324_763_204_5e0)
                * r
                + 5.769_497_221_460_691e0)
                * r
                + 4.630_337_846_156_546e0)
                * r
                + 1.423_437_110_749_683_5e0;

            den = ((((((1.050_750_071_644_416_9e-9 * r
                + 5.475_938_084_995_345e-4)
                * r
                + 1.519_866_656_361_645_7e-2)
                * r
                + 1.481_039_764_274_800_8e-1)
                * r
                + 6.897_673_349_851e-1)
                * r
                + 1.676_384_830_183_803_8e0)
                * r
                + 2.053_191_626_637_759e0)
                * r
                + 1.0;
        } else {
            r -= 5.0;

            num = ((((((2.010_334_399_292_288_1e-7 * r
                + 2.711_555_568_743_487_6e-5)
                * r
                + 1.242_660_947_388_078_4e-3)
                * r
                + 2.653_218_952_657_612_4e-2)
                * r
                + 2.965_605_718_285_048_7e-1)
                * r
                + 1.784_826_539_917_291_3e0)
                * r
                + 5.463_784_911_164_114e0)
                * r
                + 6.657_904_643_501_103e0;

            den = ((((((2.044_263_103_389_939_7e-15 * r
                + 1.421_511_758_316_446e-7)
                * r
                + 1.846_318_317_510_054_8e-5)
                * r
                + 7.868_691_311_456_133e-4)
                * r
                + 1.487_536_129_085_061_5e-2)
                * r
                + 1.369_298_809_227_358e-1)
                * r
                + 5.998_322_065_558_88e-1)
                * r
                + 1.0;
        }

        let mut x = num / den;
        if q < 0.0 {
            x = -x;
        }

        self.mean + (x * self.stdev)
    }
}

impl PDF<f64> for Normal {
    /// Compute the probability density function for the given
    /// normal distribution at the given `x` value.
    fn pdf(&self, x: f64) -> f64 {
        (-0.5 * ((x - self.mean) / self.stdev).powi(2)).exp()
            / (self.stdev * (2.0 * std::f64::consts::PI).sqrt())
    }
}
