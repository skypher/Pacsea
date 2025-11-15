//! Global test utilities for ensuring test isolation.

#[cfg(test)]
use std::sync::{Mutex, OnceLock};

#[cfg(test)]
/// Global mutex for tests that modify the PATH environment variable.
///
/// Since `std::env::set_var` affects the entire process, all tests that
/// modify PATH must serialize their execution using this mutex to prevent
/// race conditions between parallel tests.
static PATH_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

#[cfg(test)]
/// Global mutex for tests that modify the HOME environment variable.
static HOME_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

#[cfg(test)]
/// Acquire the global PATH mutex to safely modify PATH environment variable.
///
/// Output:
/// - `MutexGuard<()>` that must be held while PATH is modified.
///
/// Details:
/// - Automatically recovers from poisoned mutex (from panicked tests).
/// - Hold this guard for the entire duration that PATH is modified.
///
/// Example:
/// ```ignore
/// let _path_guard = crate::test_utils::lock_path_mutex();
/// let old_path = std::env::var("PATH").unwrap_or_default();
/// unsafe { std::env::set_var("PATH", &new_path); }
/// // do test work...
/// unsafe { std::env::set_var("PATH", &old_path); }
/// // _path_guard automatically released here
/// ```
pub fn lock_path_mutex() -> std::sync::MutexGuard<'static, ()> {
    PATH_MUTEX
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

#[cfg(test)]
/// Acquire the global HOME mutex to safely modify HOME environment variable.
pub fn lock_home_mutex() -> std::sync::MutexGuard<'static, ()> {
    HOME_MUTEX
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}
