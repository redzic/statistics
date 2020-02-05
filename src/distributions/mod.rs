//! Implements probability distributions and functions related to them,
//! such as the cumulative distribution function, probability density function,
//! as well as computations of various statistical measures, such as mean,
//! median, variance, etc.

pub mod binomial;
pub mod degenerate;
pub mod exponential;
pub mod normal;
pub mod poisson;
pub mod t;
pub mod triangular;
pub mod uniform;
pub mod weibull;

pub use super::distributions::binomial::*;
pub use super::distributions::degenerate::*;
pub use super::distributions::exponential::*;
pub use super::distributions::normal::*;
pub use super::distributions::poisson::*;
pub use super::distributions::t::*;
pub use super::distributions::triangular::*;
pub use super::distributions::uniform::*;
pub use super::distributions::weibull::*;
