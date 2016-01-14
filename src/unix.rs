//! Implementation of `errno` functionality for Unix systems.
//!
//! Adapted from `src/libstd/sys/unix/os.rs` in the Rust distribution.

// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ffi::CStr;
use std::fmt;
use std::str;
use libc::{self, c_char, c_int};

/// Wraps a platform-specific error code.
///
/// The `Display` instance maps the code to a human-readable string. It
/// calls [`strerror_r`][1] under POSIX, and [`FormatMessageW`][2] on
/// Windows.
///
/// [1]: http://pubs.opengroup.org/onlinepubs/009695399/functions/strerror.html
/// [2]: https://msdn.microsoft.com/en-us/library/windows/desktop/ms679351%28v=vs.85%29.aspx
#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Debug)]
pub struct Errno(pub c_int);

impl fmt::Display for Errno {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = [0 as c_char; 1024];
        unsafe {
            if strerror_r(self.0, buf.as_mut_ptr(), buf.len() as libc::size_t) < 0 {
                let Errno(fm_err) = errno();
                if fm_err != libc::ERANGE {
                    return write!(fmt,
                                  "OS Error {} (strerror_r returned error {})",
                                  self.0, fm_err);
                }
            }
        }
        let c_str = unsafe { CStr::from_ptr(buf.as_ptr()) };
        fmt.write_str(str::from_utf8(c_str.to_bytes()).unwrap())
    }
}

/// Returns the platform-specific value of `errno`.
pub fn errno() -> Errno {
    unsafe {
        Errno(*errno_location())
    }
}

/// Sets the platform-specific value of `errno`.
pub fn set_errno(Errno(errno): Errno) {
    unsafe {
        *errno_location() = errno;
    }
}

extern {
    #[cfg_attr(any(target_os = "macos",
                   target_os = "ios",
                   target_os = "freebsd"),
               link_name = "__error")]
    #[cfg_attr(target_os = "dragonfly",
               link_name = "__dfly_error")]
    #[cfg_attr(any(target_os = "openbsd", target_os = "bitrig", target_os = "android"),
               link_name = "__errno")]
    #[cfg_attr(target_os = "linux",
               link_name = "__errno_location")]
    fn errno_location() -> *mut c_int;

    #[cfg_attr(target_os = "linux", link_name = "__xpg_strerror_r")]
    fn strerror_r(errnum: c_int, buf: *mut c_char,
                  buflen: libc::size_t) -> c_int;
}
