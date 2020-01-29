//! Statistical library in Rust

pub mod distributions;
pub mod functions;
pub mod stats;

pub use crate::distributions::binomial::*;
pub use crate::distributions::normal::*;
pub use crate::distributions::poisson::*;
pub use crate::distributions::t::*;
pub use crate::distributions::uniform::*;
pub use crate::functions::erf::*;
pub use crate::stats::calculate::*;
pub use crate::stats::traits::*;
