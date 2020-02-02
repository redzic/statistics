//! This crate is a
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
