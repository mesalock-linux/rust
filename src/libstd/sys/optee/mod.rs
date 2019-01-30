//! System bindings for the OP-TEE Trusted OS platform
//!
//! This module contains the facade (aka platform-specific) implementations of
//! OS level functionality for OP-TEE Trusted OS.

use crate::os::raw::c_char;

pub mod alloc;
pub mod args;
#[cfg(feature = "backtrace")]
pub mod backtrace;
pub mod cmath;
pub mod condvar;
pub mod env;
pub mod fs;
pub mod io;
pub mod memchr;
pub mod mutex;
pub mod net;
pub mod os;
pub mod os_str;
pub mod path;
pub mod pipe;
pub mod process;
pub mod rwlock;
pub mod stack_overflow;
pub mod stdio;
pub mod thread;
pub mod thread_local;
pub mod time;

#[cfg(not(test))]
pub fn init() {
}

pub fn unsupported<T>() -> crate::io::Result<T> {
    Err(unsupported_err())
}

pub fn unsupported_err() -> crate::io::Error {
    crate::io::Error::new(crate::io::ErrorKind::Other,
                   "operation not supported on optee yet")
}

pub fn decode_error_kind(_code: i32) -> crate::io::ErrorKind {
    crate::io::ErrorKind::Other
}

// This enum is used as the storage for a bunch of types which can't actually
// exist.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Void {}

pub unsafe fn strlen(mut _s: *const c_char) -> usize {
    0
}

pub unsafe fn abort_internal() -> ! {
    loop { }
}

pub fn hashmap_random_keys() -> (u64, u64) {
    (1, 2)
}
