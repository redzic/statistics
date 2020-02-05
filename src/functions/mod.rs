//! Provides various utility statistical functions (e.g. gamma, erf, etc.)

pub mod beta;
pub mod erf;
pub mod factorial;
pub mod gamma;

pub use super::functions::beta::*;
pub use super::functions::erf::*;
pub use super::functions::factorial::*;
pub use super::functions::gamma::*;
