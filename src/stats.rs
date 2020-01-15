use rayon::prelude::*;

pub fn mean(data: &[f64]) -> f64 {
    data.iter().sum::<f64>() / data.len() as f64
}

pub fn median(data: &[f64]) -> f64 {
    let n = data.len();
    let mut _data = data.to_owned();

    _data.par_sort_unstable_by(|x, y| x.partial_cmp(&y).unwrap());

    match n % 2 {
        0 => (_data[n / 2 - 1] + _data[n / 2]) / 2.0,
        _ => _data[n / 2],
    }
}

pub fn variance(data: &[f64]) -> f64 {
    let mean = mean(data);
    data.iter().map(|i| (i - mean).powi(2)).sum::<f64>() / (data.len() as f64 - 1.0)
}

pub fn stdev(data: &[f64]) -> f64 {
    variance(data).sqrt()
}

fn erf_impl(z: f64) -> f64 {
    const P: f64 = 0.3275911;
    const A_1: f64 = 0.254829592;
    const A_2: f64 = -0.284496736;
    const A_3: f64 = 1.421413741;
    const A_4: f64 = -1.453152027;
    const A_5: f64 = 1.061405429;

    let t: f64 = 1.0 / (1.0 + P * z);

    1.0 - (A_1 * t
        + A_2 * t.powi(2)
        + A_3 * t.powi(3)
        + A_4 * t.powi(4)
        + A_5 * t.powi(5))
        * std::f64::consts::E.powf(-z.powi(2))
}

pub fn erf(z: f64) -> f64 {
    // TODO use arbitrary precision algorithm
    if z > 0.0 { erf_impl(z) } else { -erf_impl(-z) }
}

pub fn sgn(x: f64) -> f64 {
    match x.partial_cmp(&0f64).unwrap() {
        std::cmp::Ordering::Equal => 0.0,
        std::cmp::Ordering::Greater => 1.0,
        std::cmp::Ordering::Less => -1.0,
    }
}

pub fn inverse_erf(x: f64) -> f64 {
    // TODO switch to a = 0.147 (change other consts too that depend on A's value)
    const A: f64 = 0.14001_22886_866666;
    const FRAC_2_PI_A: f64 = 4.54688_49794_48285;
    let ln_one_minus_x_squared = (1.0 - x.powi(2)).ln();
    let y = FRAC_2_PI_A + ln_one_minus_x_squared / 2.0;

    sgn(x) * ((y.powi(2) - ln_one_minus_x_squared / A).sqrt() - y).sqrt()
}
