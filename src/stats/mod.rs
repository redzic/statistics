//! Provides traits and implementations for various statistical measures (e.g. mean, median, variance, standard deviation, etc.)

pub mod calculate;
pub mod traits;

pub use super::stats::calculate::*;
pub use super::stats::traits::*;
