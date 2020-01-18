fn _erf_approx_impl(z: f64) -> f64 {
    const P: f64 = 0.327_591_1;
    const A_1: f64 = 0.254_829_592;
    const A_2: f64 = -0.284_496_736;
    const A_3: f64 = 1.421_413_741;
    const A_4: f64 = -1.453_152_027;
    const A_5: f64 = 1.061_405_429;

    let t: f64 = 1.0 / (1.0 + P * z);

    1.0 - (A_1 * t
        + A_2 * t.powi(2)
        + A_3 * t.powi(3)
        + A_4 * t.powi(4)
        + A_5 * t.powi(5))
        * std::f64::consts::E.powf(-z.powi(2))
}

fn _erf_approx(z: f64) -> f64 {
    match z.partial_cmp(&0f64).unwrap() {
        std::cmp::Ordering::Greater => _erf_approx_impl(z),
        _ => -_erf_approx_impl(-z),
    }
}

fn _erf_taylor(x: f64, mx2: f64) -> f64 {
    x * (1.1283791670955125739
        + mx2 * (0.37612638903183752464 + mx2 * 0.11283791670955125739))
}

pub fn erf(x: f64) -> f64 {
    let mx2 = -x * x;

    // underflow
    if mx2 < -750.0 {
        match x.partial_cmp(&0f64).unwrap() {
            std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => 1.0,
            _ => -1.0,
        };
        if x >= 0.0 {
            return 1.0;
        } else {
            return -1.0;
        }
    }

    if x >= 0.0 {
        if x < 5e-3f64 {
            return _erf_taylor(x, mx2);
        }

        // These are fallback cases if we can't use the taylor approximation
        return _erf_approx(x);
    // return 1.0 - mx2.exp() * erfcx(x);
    // https://github.com/scipy/scipy/blob/8dba340293fe20e62e173bdf2c10ae208286692f/scipy/special/Faddeeva.cc#L174
    } else {
        // x < 0
        if x < -5e-3f64 {
            return _erf_taylor(x, mx2);
        }
        return _erf_approx(x);
        // return mx2.exp() * erfcx(-x) - 1.0;
    }
}

pub fn sign(x: f64) -> f64 {
    match x.partial_cmp(&0f64).unwrap() {
        std::cmp::Ordering::Equal => 0.0,
        std::cmp::Ordering::Greater => 1.0,
        std::cmp::Ordering::Less => -1.0,
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
