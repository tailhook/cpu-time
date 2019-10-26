use std::io::Result;
use std::marker::PhantomData;
use std::rc::Rc;
use std::time::Duration;

use winapi::shared::minwindef::{BOOL, FILETIME};
use winapi::um::processthreadsapi::{GetCurrentProcess, GetCurrentThread};
use winapi::um::processthreadsapi::{GetProcessTimes, GetThreadTimes};

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

fn to_duration(kernel_time: FILETIME, user_time: FILETIME) -> Duration {
    // resolution: 100ns
    let kns100 = ((kernel_time.dwHighDateTime as u64) << 32) + kernel_time.dwLowDateTime as u64;
    let uns100 = ((user_time.dwHighDateTime as u64) << 32) + user_time.dwLowDateTime as u64;
    return Duration::new(
        (kns100 + uns100) / 10_000_000,
        (((kns100 + uns100) * 100) % 1000_000_000) as u32,
    );
}

fn zero() -> FILETIME {
    FILETIME {
        dwLowDateTime: 0,
        dwHighDateTime: 0,
    }
}

impl ProcessTime {
    /// Get current CPU time used by a process
    pub fn try_now() -> Result<Self> {
        let mut kernel_time = zero();
        let mut user_time = zero();
        let process = unsafe { GetCurrentProcess() };
        let ok = unsafe { GetProcessTimes(process,
            &mut zero(), &mut zero(),
            &mut kernel_time, &mut user_time) };
        if ok == 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(Self(to_duration(kernel_time, user_time)))
    }

    /// Get current CPU time used by a process
    ///
    /// # Panics
    ///
    /// If GetProcessTimes fails. This may happen, for instance, in case of
    /// insufficient permissions.
    ///
    /// See [this MSDN page][msdn] for details. If you prefer to handle such errors, consider
    /// using `try_now`.
    ///
    /// [msdn]: https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocesstimes
    pub fn now() -> Self {
        Self::try_now().expect("GetProcessTimes failed")
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
        let mut kernel_time = zero();
        let mut user_time = zero();
        let thread = unsafe { GetCurrentThread() };
        let ok = unsafe { GetThreadTimes(thread,
            &mut zero(), &mut zero(),
            &mut kernel_time, &mut user_time) };
        if ok == 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(Self(to_duration(kernel_time, user_time), PhantomData))
    }

    ///
    /// # Panics
    ///
    /// If GetThreadTimes fails. This may happen, for instance, in case of
    /// insufficient permissions.
    ///
    /// See [this MSDN page][msdn] for details. If you prefer to handle such errors, consider
    /// using `try_now`.
    ///
    /// [msdn]: https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getthreadtimes
    pub fn now() -> Self {
        Self::try_now().expect("GetThreadTimes failed")
    }

    /// Returns the amount of CPU time used from the previous timestamp to now.
    pub fn try_elapsed(&self) -> Result<Duration> {
        Ok(Self::try_now()?.duration_since(*self))
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
