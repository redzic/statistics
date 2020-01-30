use super::factorial::*;
use super::gamma::*;

// NOTE: this function doesn't work
// this formatting makes the most sense in this case
#[allow(non_snake_case)]
pub fn hyper_2F1(a: f64, b: f64, c: f64, z: f64) -> f64 {
    // TODO add checks for values

    const N: usize = 1000;
    let R_J: Vec<f64> = (2..N)
        .map(|j| {
            ((a + j as f64 - 1.0) * (b + j as f64 - 1.0))
                / (j as f64 * (c + j as f64 - 1.0))
        })
        .collect();

    let mut S: Vec<f64> = vec![1.0, z * ((a * b) / c)];

    for j in 2..N - 2 {
        S.push(S[j - 1] + (S[j - 1] - S[j - 2]) * R_J[j - 2] * z);
    }

    S.iter().sum()
}
