//! CPU Time Measurement Library
//! ============================
//!
//! [Documentation](https://docs.rs/cpu-time) |
//! [Github](https://github.com/tailhook/cpu-time) |
//! [Crate](https://crates.io/crates/cpu-time)
//!
//! # Example
//!
//! ```rust
//!
//! use std::time::Duration;
//! use cpu_time::ProcessTime;
//!
//! let start = ProcessTime::now();
//! // .. do something ..
//! let cpu_time: Duration = start.elapsed();
//! println!(" {:?}", cpu_time);
//!
//! ```

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

extern crate libc;

// It looks like all modern unixes support clock_gettime(..CPUTIME..)
#[cfg(unix)]
mod clock_gettime;

#[cfg(unix)]
pub use clock_gettime::{ProcessTime, ThreadTime};
