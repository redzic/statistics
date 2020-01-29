use libm::tgamma;

pub trait Gamma {
    fn gamma(&self) -> f64;
    // fn gamma_li(&self) -> f64;
    // fn gamma_ui(&self) -> f64;
}

impl Gamma for f64 {
    fn gamma(&self) -> f64 {
        // TODO copy code here
        // We don't need the ENTIRE libm port
        // we only need a handful of functions
        tgamma(*self)
    }
}
