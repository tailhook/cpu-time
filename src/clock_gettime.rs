use std::time::Duration;
use std::rc::Rc;
use std::marker::PhantomData;

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
pub struct ThreadTime(Duration,
                      // makes type non-sync and non-send
                      PhantomData<Rc<()>>);


impl ProcessTime {
    /// Get current CPU time used by a process process
    ///
    /// # Panics
    ///
    /// This method panics if linux kernel doesn't support
    /// CLOCK_PROCESS_CPUTIME_ID, which works since linux 2.6.12 (~ year 2005).
    pub fn now() -> ProcessTime {
        let mut time = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        if unsafe { clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &mut time) } == -1
        {
            panic!("Process CPU time is not supported");
        }
        ProcessTime(Duration::new(time.tv_sec as u64, time.tv_nsec as u32))
    }
    /// Returns the amount of CPU time used from the previous timestamp to now.
    pub fn elapsed(&self) -> Duration {
        ProcessTime::now().duration_since(*self)
    }
    /// Returns the amount of CPU time used from the previous timestamp.
    pub fn duration_since(&self, timestamp: ProcessTime) -> Duration {
        self.0 - timestamp.0
    }

    /// Returns the total amount of CPU time used from the program start.
    pub fn as_duration(&self) -> Duration {
        self.0
    }
}

impl ThreadTime {
    /// Get current CPU time used by a process process
    ///
    /// # Panics
    ///
    /// This method panics if linux kernel doesn't support
    /// CLOCK_THREAD_CPUTIME_ID, which works since linux 2.6.12 (~ year 2005).
    pub fn now() -> ThreadTime {
        let mut time = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        if unsafe { clock_gettime(CLOCK_THREAD_CPUTIME_ID, &mut time) } == -1
        {
            panic!("Process CPU time is not supported");
        }
        ThreadTime(Duration::new(time.tv_sec as u64, time.tv_nsec as u32),
                   PhantomData)
    }
    /// Returns the amount of CPU time used by the current thread
    /// from the previous timestamp to now.
    pub fn elapsed(&self) -> Duration {
        ThreadTime::now().duration_since(*self)
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
