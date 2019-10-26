use std::io::{Result, Error};
use std::marker::PhantomData;
use std::rc::Rc;
use std::time::Duration;

use libc::{clock_gettime, timespec};
use libc::{CLOCK_PROCESS_CPUTIME_ID, CLOCK_THREAD_CPUTIME_ID};

/// CPU Time Used by The Whole Process
///
/// This is an opaque type similar to `std::time::Instant`.
/// Use `elapsed()` or `duration_since()` to get meaningful time deltas.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct ProcessTime(Duration);

/// CPU Time Used by The Current Thread
///
/// This is an opaque type similar to `std::time::Instant`.
/// Use `elapsed()` or `duration_since()` to get meaningful time deltas.
///
/// This type is non-thread-shareable (!Sync, !Send) because otherwise it's
/// to easy to mess up times from different threads. However, you can freely
/// send Duration's returned by `elapsed()` and `duration_since()`.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct ThreadTime(
    Duration,
    // makes type non-sync and non-send
    PhantomData<Rc<()>>,
);

impl ProcessTime {
    /// Get current CPU time used by a process process
    pub fn try_now() -> Result<Self> {
        let mut time = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        if unsafe { clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &mut time) } == -1
        {
            return Err(Error::last_os_error());
        }
        Ok(ProcessTime(Duration::new(
            time.tv_sec as u64,
            time.tv_nsec as u32,
        )))
    }

    /// Get current CPU time used by a process
    ///
    /// # Panics
    ///
    /// If `CLOCK_THREAD_CPUTIME_ID` is not supported by the kernel.
    ///
    /// On Linux, it was added in version 2.6.12 (year 2005). \
    /// [On OpenBSD][openbsd] & [FreeBSD][freebsd] support was added in 2013. \
    /// [On MacOS][macos], `clock_gettime` was not supported until Sierra (2016).
    ///
    /// [openbsd]: https://github.com/openbsd/src/commit/7b36c281ba1c99d528efca950572c207acd2e184
    /// [freebsd]: https://github.com/freebsd/freebsd/commit/e8cf8aab231fe1b1ae82eff6e64af146514eea71
    /// [macos]: http://www.manpagez.com/man/3/clock_gettime/
    pub fn now() -> Self {
        Self::try_now().expect("CLOCK_PROCESS_CPUTIME_ID unsupported")
    }

    /// Returns the amount of CPU time used from the previous timestamp to now.
    pub fn try_elapsed(&self) -> Result<Duration> {
        Ok(Self::try_now()?.duration_since(*self))
    }

    /// Returns the amount of CPU time used from the previous timestamp to now.
    ///
    /// # Panics
    ///
    /// If `ProcessTime::now()` panics.
    pub fn elapsed(&self) -> Duration {
        Self::now().duration_since(*self)
    }

    /// Returns the amount of CPU time used from the previous timestamp.
    pub fn duration_since(&self, timestamp: Self) -> Duration {
        self.0 - timestamp.0
    }

    /// Returns the total amount of CPU time used from the program start.
    pub fn as_duration(&self) -> Duration {
        self.0
    }
}

impl ThreadTime {
    /// Get current CPU time used by a process process
    pub fn try_now() -> Result<Self> {
        let mut time = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        if unsafe { clock_gettime(CLOCK_THREAD_CPUTIME_ID, &mut time) } == -1
        {
            return Err(Error::last_os_error());
        }
        Ok(ThreadTime(
            Duration::new(time.tv_sec as u64, time.tv_nsec as u32),
            PhantomData,
        ))
    }

    /// Get current CPU time used by a process
    ///
    /// # Panics
    ///
    /// If `CLOCK_THREAD_CPUTIME_ID` is not supported by the kernel.
    ///
    /// On Linux, it was added in version 2.6.12 (year 2005). \
    /// [On OpenBSD][openbsd] & [FreeBSD][freebsd] support was added in 2013. \
    /// [On MacOS][macos], `clock_gettime` was not supported until Sierra (2016).
    ///
    /// [openbsd]: https://github.com/openbsd/src/commit/7b36c281ba1c99d528efca950572c207acd2e184
    /// [freebsd]: https://github.com/freebsd/freebsd/commit/e8cf8aab231fe1b1ae82eff6e64af146514eea71
    /// [macos]: http://www.manpagez.com/man/3/clock_gettime/
    pub fn now() -> Self {
        Self::try_now().expect("CLOCK_PROCESS_CPUTIME_ID unsupported")
    }

    /// Returns the amount of CPU time used by the current thread
    /// from the previous timestamp to now.
    pub fn try_elapsed(&self) -> Result<Duration> {
        Ok(ThreadTime::try_now()?.duration_since(*self))
    }

    /// Returns the amount of CPU time used from the previous timestamp to now.
    ///
    /// # Panics
    ///
    /// If `ThreadTime::now()` panics.
    pub fn elapsed(&self) -> Duration {
        Self::now().duration_since(*self)
    }

    /// Returns the amount of CPU time used by the current thread
    /// from the previous timestamp.
    pub fn duration_since(&self, timestamp: ThreadTime) -> Duration {
        self.0 - timestamp.0
    }

    /// Returns the total amount of CPU time used from the program start.
    pub fn as_duration(&self) -> Duration {
        self.0
    }
}
