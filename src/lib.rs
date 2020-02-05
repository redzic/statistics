//! This crate provides various tools for statistical computation that
//! can be very difficult to manually implement. It provides various
//! probability distributions and methods to calculate things such as
//! the cumulative density function, percent-point function (inverse CDF),
//! probability density function, and more.
//!
//! # Example
//! The following example samples from a standard normal distribution
//!
//! ```
//! use statistics::distributions::*;
//! use statistics::stats::*;
//!
//! # fn main() {
//! let n = Normal::new(0.0, 1.0);
//! println!("{}", n.cdf(-1.0));
//! // percent-point function (inverse cdf)
//! println!("{}", n.ppf(1.0));
//! # }
//! ```

pub mod distributions;
pub mod functions;
pub mod stats;
