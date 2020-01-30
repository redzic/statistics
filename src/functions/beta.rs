use crate::functions::gamma::Gamma;
use special::Beta;

pub fn beta(x: f64, y: f64) -> f64 {
    // TODO add check for negative integers
    (x.gamma() * y.gamma()) / (x + y).gamma()
}

// TODO bring over implementation of beta over here
// That crate isn't maintained anyway and is too small
// of a dependency to matter
pub fn beta_inc(a: f64, b: f64, x: f64) -> f64 {
    x.inc_beta(a, b, beta(a, b).ln())
}
