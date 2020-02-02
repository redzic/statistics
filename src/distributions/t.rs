use crate::functions::beta::*;
use crate::functions::gamma::*;
use crate::stats::traits::*;

pub struct T {
    // degrees of freedom
    df: i32,
}

impl T {
    pub fn new(df: i32) -> Self {
        T { df }
    }
}

impl PDF<f64> for T {
    fn pdf(&self, t: f64) -> f64 {
        let df = self.df as f64;
        ((0.5 * (df + 1.0)).gamma()
            / ((df * std::f64::consts::PI).sqrt() * (0.5 * df).gamma()))
            * (1.0 + t.powi(2) / df).powf(-0.5 * (df + 1.0))
    }
}

impl CDF<f64> for T {
    fn cdf(&self, q: f64) -> f64 {
        // port of SciPy's implementation, because it is much
        // faster than the previous one
        const MACHEP: f64 = 1.11022302462515654042e-16;

        if self.df <= 0 {
            return std::f64::NAN;
        }

        if q == 0.0 {
            return 0.5;
        }

        let rk: f64;
        let z: f64;
        let mut p: f64;

        if q < -2.0 {
            rk = self.df as f64;
            z = rk / (rk + q * q);
            p = 0.5 * beta_inc(0.5 * rk, 0.5, z);
            return p;
        }

        // compute integral from -t to +t
        let x: f64;

        if q < 0.0 {
            x = -q;
        } else {
            x = q;
        }

        // degrees of freedom
        rk = self.df as f64;
        z = 1.0 + (x * x) / rk;

        // test if k is odd or even
        let mut f: f64;
        let mut tz: f64;
        let mut j: i32;

        let xsqk: f64;
        if (self.df & 1) != 0 {
            // computation for odd k
            xsqk = x / rk.sqrt();
            p = xsqk.atan();
            if self.df > 1 {
                f = 1.0;
                tz = 1.0;
                j = 3;
                while (j <= (self.df - 2)) && ((tz / f) > MACHEP) {
                    tz *= (j - 1) as f64 / (z * j as f64);
                    f += tz;
                    j += 2;
                }
                p += f * xsqk / z;
            }
            p *= 2.0 / std::f64::consts::PI;
        } else {
            // computation for even k
            f = 1.0;
            tz = 1.0;
            j = 2;
            while (j <= (self.df - 2)) && ((tz / f) > MACHEP) {
                tz *= (j - 1) as f64 / (z * j as f64);
                f += tz;
                j += 2;
            }
            p = f * x / (z * rk).sqrt();
        }

        // common exit
        if q < 0.0 {
            // note destructino of relative accuracy
            p = -p;
        }

        return 0.5 + 0.5 * p;
    }
}

impl PPF<f64> for T {
    fn ppf(&self, q: f64) -> f64 {
        // TODO switch to better algorithm than guess and check
        // find the value p such that self.cdf(p) = q

        const EPSILON: f64 = 1.0e-12f64;
        if q == 0.5 {
            return 0.0;
        }

        // TODO correctly determine min/max bounds
        let mut bound = 1000000.0;
        loop {
            if self.pdf(bound) < EPSILON {
                break;
            } else {
                bound += 1000000.0;
            }
        }

        let mut low: f64;
        let mut high: f64;

        if q > 0.5 {
            low = 0.0;
            high = bound;
        } else {
            low = -bound;
            high = 0.0;
        }

        let mut p = 0.5 * (high + low);

        while (self.cdf(p) - q).abs() > EPSILON {
            if self.cdf(p) < q {
                low = p;
            } else {
                high = p;
            }
            p = 0.5 * (high + low);
        }

        p
    }
}
