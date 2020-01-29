use crate::functions::gamma::Gamma;

pub fn beta(x: f64, y: f64) -> f64 {
    (x.gamma() * y.gamma()) / (x + y).gamma()
}
