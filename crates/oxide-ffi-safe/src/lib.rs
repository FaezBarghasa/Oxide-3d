//! Safe RAII boundary wrappers around foreign C/C++ driver pointers and handles.

use oxide_hal::ComputeError;
use std::panic::{AssertUnwindSafe, catch_unwind};

/// Safely execute an FFI block while catching potential unhandled foreign panics or aborts.
pub fn catch_ffi_boundary<F, R>(f: F) -> Result<R, ComputeError>
where
    F: FnOnce() -> Result<R, ComputeError>,
{
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(res) => res,
        Err(_) => Err(ComputeError::DriverError(
            "Foreign FFI panicked".to_string(),
        )),
    }
}
