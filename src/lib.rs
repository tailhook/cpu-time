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

#[cfg(target_os="linux")]
mod linux;

#[cfg(target_os="linux")]
pub use linux::{ProcessTime, ThreadTime};
