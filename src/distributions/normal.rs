use crate::functions::erf::*;
use crate::stats::traits::*;
use std::cmp::PartialEq;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Normal {
    mu: f64,
    sigma: f64,
}

impl Normal {
    pub fn new(mu: f64, sigma: f64) -> Self {
        if sigma <= 0.0 {
            panic!("σ must be positive");
        }

        Normal { mu, sigma }
    }

    pub fn from(data: &[f64]) -> Self {
        Normal {
            mu: data.mean(),
            sigma: data.stdev(),
        }
    }
}

impl Add<Normal> for Normal {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            mu: self.mu + other.mu,
            sigma: (self.sigma.powi(2) + other.sigma.powi(2)).sqrt(),
        }
    }
}

impl Add<f64> for Normal {
    type Output = Self;

    fn add(self, scalar: f64) -> Self {
        Self {
            mu: self.mu + scalar,
            sigma: self.sigma,
        }
    }
}

impl Mul<f64> for Normal {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            mu: self.mu * scalar,
            sigma: self.sigma * scalar,
        }
    }
}

// Clippy is wrong in this case
#[allow(clippy::suspicious_arithmetic_impl)]
impl Sub<Normal> for Normal {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            mu: self.mu - other.mu,
            sigma: (self.sigma.powi(2) + other.sigma.powi(2)).sqrt(),
        }
    }
}

impl Sub<f64> for Normal {
    type Output = Self;

    fn sub(self, scalar: f64) -> Self {
        Self {
            mu: self.mu - scalar,
            sigma: self.sigma,
        }
    }
}

impl PartialEq for Normal {
    fn eq(&self, other: &Self) -> bool {
        self.mu == other.mu && self.sigma == other.sigma
    }
}

impl Mean<f64> for Normal {
    fn mean(&self) -> f64 {
        self.mu
    }
}

impl Median<f64> for Normal {
    fn median(&self) -> f64 {
        self.mu
    }
}

impl Mode<f64> for Normal {
    fn mode(&self) -> f64 {
        self.mu
    }
}

impl StdDev<f64> for Normal {
    fn stdev(&self) -> f64 {
        self.sigma
    }
}

impl Variance<f64> for Normal {
    fn variance(&self) -> f64 {
        self.sigma.powi(2)
    }
}

impl CDF<f64> for Normal {
    fn cdf(&self, x: f64) -> f64 {
        0.5 * (1.0
            + ((x - self.mu) / (self.sigma * std::f64::consts::SQRT_2)).erf())
    }
}

impl PPF for Normal {
    /// Compute the inverse cumulative distribution function for the given Normal
    /// distribution. Returns NaN if p < 0.0 or p > 1.0.
    fn ppf(&self, a: f64) -> f64 {
        let q = a - 0.5;

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
            return self.mu + (x * self.sigma);
        }

        let mut r = if q <= 0.0 { a } else { 1.0 - a };
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

        self.mu + (x * self.sigma)
    }
}

impl PDF<f64> for Normal {
    /// Compute the probability density function for the given
    /// normal distribution at the given `x` value.
    fn pdf(&self, x: f64) -> f64 {
        (-0.5 * ((x - self.mu) / self.sigma).powi(2)).exp()
            / (self.sigma * (2.0 * std::f64::consts::PI).sqrt())
    }
}

impl Skewness<f64> for Normal {
    fn skewness(&self) -> f64 {
        0.0
    }
}
