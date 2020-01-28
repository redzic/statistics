use libc::c_double;
use std::cmp::Ordering;

extern "C" {
    fn erf(x: c_double) -> c_double;
    fn erfc(x: c_double) -> c_double;
    fn tgamma(x: c_double) -> c_double;
}

pub trait Error {
    fn erf(&self) -> f64;
    fn erfc(&self) -> f64;
    fn inv_erf(&self) -> f64;
}

// TODO try implementing C version in Rust?

impl Error for f64 {
    #[inline]
    fn erf(&self) -> f64 {
        unsafe { erf(*self) as Self }
    }

    #[inline]
    fn erfc(&self) -> f64 {
        1.0 - (*self).erf()
    }

    #[inline]
    fn inv_erf(&self) -> f64 {
        inv_erf(*self)
    }
}

#[inline]
fn sign(x: f64) -> f64 {
    match x.partial_cmp(&0f64).unwrap() {
        Ordering::Equal => 0.0,
        Ordering::Greater => 1.0,
        Ordering::Less => -1.0,
    }
}

#[inline]
fn inv_erf(x: f64) -> f64 {
    // TODO switch to a = 0.147 (change other consts too that depend on A's value)
    const A: f64 = 0.140_012_288_686_666_6;
    const FRAC_2_PI_A: f64 = 4.54688_49794_48285;
    let ln_one_minus_x_squared = (1.0 - x.powi(2)).ln();
    let y = FRAC_2_PI_A + ln_one_minus_x_squared / 2.0;

    sign(x) * ((y.powi(2) - ln_one_minus_x_squared / A).sqrt() - y).sqrt()
}
