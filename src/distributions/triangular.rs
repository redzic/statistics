//! Implementation of the [Triangular](https://en.wikipedia.org/wiki/Triangular_distribution) distribution

use crate::stats::traits::*;

/// Triangular distribution
#[derive(Debug, Copy, Clone)]
pub struct Triangular {
    a: f64,
    b: f64,
    c: f64,
}

impl Triangular {
    /// Creates new Triangular distribution with parameters `a`, `b`, and `c`.
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        if !(a < b) {
            panic!("a < b must be true");
        }

        if !(a <= c && c <= b) {
            panic!("a <= c <= b must be true");
        }
        Triangular { a, b, c }
    }
}

impl PDF<f64> for Triangular {
    fn pdf(&self, x: f64) -> f64 {
        if x < self.a {
            return 0.0;
        } else if self.a <= x && x < self.c {
            return (2.0 * (x - self.a))
                / ((self.b - self.a) * (self.c - self.a));
        } else if x == self.c {
            return 2.0 / (self.b - self.a);
        } else if self.c < x && x <= self.b {
            return (2.0 * (self.b - x))
                / ((self.b - self.a) * (self.b - self.c));
        } else {
            0.0
        }
    }
}

impl CDF<f64> for Triangular {
    fn cdf(&self, x: f64) -> f64 {
        if x < self.a {
            return 0.0;
        } else if self.a <= x && x <= self.c {
            return (x - self.a).powi(2)
                / ((self.b - self.a) * (self.c - self.a));
        } else if self.c < x && x < self.b {
            return 1.0
                - (self.b - x).powi(2) / ((self.b - self.a) * (self.b - self.c));
        } else {
            1.0
        }
    }
}

impl Mean<f64> for Triangular {
    fn mean(&self) -> f64 {
        return (self.a + self.b + self.c) / 3.0;
    }
}

impl Median<f64> for Triangular {
    fn median(&self) -> f64 {
        if self.c >= (self.a + self.b) / 2.0 {
            self.a + (((self.b - self.a) * (self.c - self.a)) / 2.0).sqrt()
        } else {
            self.b + (((self.b - self.a) * (self.b - self.c)) / 2.0).sqrt()
        }
    }
}

impl Mode<f64> for Triangular {
    fn mode(&self) -> f64 {
        self.c
    }
}

impl Variance<f64> for Triangular {
    fn variance(&self) -> f64 {
        (self.a.powi(2) + self.b.powi(2) + self.c.powi(2)
            - self.a * self.b
            - self.a * self.c
            - self.b * self.c)
            / 18.0
    }

    fn stdev(&self) -> f64 {
        self.variance().sqrt()
    }
}

impl Skewness<f64> for Triangular {
    fn skewness(&self) -> f64 {
        (std::f64::consts::SQRT_2
            * (self.a + self.b - 2.0 * self.c)
            * (2.0 * self.a - self.b - self.c)
            * (self.a - 2.0 * self.b + self.c))
            / (5.0
                * (self.a.powi(2) + self.b.powi(2) + self.c.powi(2)
                    - self.a * self.b
                    - self.a * self.c
                    - self.b * self.c)
                    .powf(1.5))
    }
}
