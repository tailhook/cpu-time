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
//! // Manually handle errors
//! let start = ProcessTime::try_now().expect("Getting process time failed");
//! // .. do something ..
//! let cpu_time: Duration = start.try_elapsed().expect("Getting process time failed");;
//! println!(" {:?}", cpu_time);
//!
//! // Panic in case of an error
//! let start = ProcessTime::now();
//! // .. do something ..
//! let cpu_time: Duration = start.elapsed();
//! println!(" {:?}", cpu_time);
//! ```

#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

#[cfg(unix)] extern crate libc;
#[cfg(windows)] extern crate winapi;

// It looks like all modern unixes support clock_gettime(..CPUTIME..)
#[cfg(unix)] mod clock_gettime;
#[cfg(windows)] mod windows;

#[cfg(unix)] pub use clock_gettime::{ProcessTime, ThreadTime};

#[cfg(windows)] pub use windows::{ProcessTime, ThreadTime};
