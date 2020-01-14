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

    let x: f64 = 1.0 / (1.0 + P * z);

    1.0 - (A_1 * x + A_2 * x.powi(2) + A_3 * x.powi(3) + A_4 * x.powi(4) + A_5 * x.powi(5))
        * std::f64::consts::E.powf(-z.powi(2))
}

pub fn erf(z: f64) -> f64 {
    // TODO use arbitrary precision algorithm
    if z > 0.0 {
        erf_impl(z)
    } else {
        -erf_impl(-z)
    }
}
