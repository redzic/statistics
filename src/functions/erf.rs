use std::cmp::Ordering;

pub trait Erf {
    fn erf(&self) -> f64;
}

impl Erf for f64 {
    fn erf(&self) -> f64 {
        if self.abs() < 5e-3f64 {
            erf_taylor(*self)
        } else {
            erf_polynomial(*self)
        }
    }
}

pub fn sign(x: f64) -> f64 {
    match x.partial_cmp(&0f64).unwrap() {
        Ordering::Equal => 0.0,
        Ordering::Greater => 1.0,
        Ordering::Less => -1.0,
    }
}

pub fn inv_erf(x: f64) -> f64 {
    // TODO switch to a = 0.147 (change other consts too that depend on A's value)
    const A: f64 = 0.140_012_288_686_666_6;
    const FRAC_2_PI_A: f64 = 4.54688_49794_48285;
    let ln_one_minus_x_squared = (1.0 - x.powi(2)).ln();
    let y = FRAC_2_PI_A + ln_one_minus_x_squared / 2.0;

    sign(x) * ((y.powi(2) - ln_one_minus_x_squared / A).sqrt() - y).sqrt()
}

fn erf_polynomial(x: f64) -> f64 {
    let t = 1.0 / (1.0 + 0.5 * x.abs());
    let tau = t
        * (-x.powi(2) - 1.265_512_23
            + 1.000_023_68 * t
            + 0.374_091_96 * t.powi(2)
            + 0.096_784_18 * t.powi(3)
            - 0.186_288_06 * t.powi(4)
            + 0.278_868_07 * t.powi(5)
            - 1.135_203_98 * t.powi(6)
            + 1.488_515_87 * t.powi(7)
            - 0.822_152_23 * t.powi(8)
            + 0.170_872_77 * t.powi(9))
        .exp();

    if x >= 0.0 { 1.0 - tau } else { tau - 1.0 }
}

fn erf_taylor(x: f64) -> f64 {
    let mx2 = -x.powi(2);
    x * (std::f64::consts::FRAC_2_SQRT_PI
        + mx2 * (0.376_126_389_031_837_54 + mx2 * 0.112_837_916_709_551_26))
}
