use std::time::Duration;
use std::rc::Rc;
use std::marker::PhantomData;

use winapi::um::processthreadsapi::{GetProcessTimes, GetThreadTimes};
use winapi::um::processthreadsapi::{GetCurrentProcess, GetCurrentThread};
use winapi::shared::minwindef::FILETIME;


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

fn to_duration(kernel_time: FILETIME, user_time: FILETIME) -> Duration {
    // resolution: 100ns
    let kns100 = ((kernel_time.dwHighDateTime as u64) << 32) +
                  kernel_time.dwLowDateTime as u64;
    let uns100 = ((user_time.dwHighDateTime as u64) << 32) +
                  user_time.dwLowDateTime as u64;
    return Duration::new(
        (kns100 + uns100) / 10_000_000,
        (((kns100 + uns100) * 100) % 1000_000_000) as u32);
}

fn zero() -> FILETIME {
    FILETIME {
        dwLowDateTime: 0,
        dwHighDateTime: 0,
    }
}

impl ProcessTime {
    /// Get current CPU time used by a process process
    ///
    /// # Panics
    ///
    /// If `GetProcessTimes` fails (not sure if it can happen)
    pub fn now() -> ProcessTime {
        let mut kernel_time = zero();
        let mut user_time = zero();
        let process = unsafe { GetCurrentProcess() };
        let ok = unsafe { GetProcessTimes(process,
            &mut zero(), &mut zero(),
            &mut kernel_time, &mut user_time) };
        if ok == 0 {
            panic!("Can't get process times");
        }
        return ProcessTime(to_duration(kernel_time, user_time));
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
    /// If `GetThreadTimes` fails (not sure if it can happen)
    pub fn now() -> ThreadTime {
        let mut kernel_time = zero();
        let mut user_time = zero();
        let thread = unsafe { GetCurrentThread() };
        let ok = unsafe { GetThreadTimes(thread,
            &mut zero(), &mut zero(),
            &mut kernel_time, &mut user_time) };
        if ok == 0 {
            panic!("Can't get trhad times");
        }
        return ThreadTime(to_duration(kernel_time, user_time), PhantomData);
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
