//! Implementation of `errno` functionality for Windows.
//!
//! Adapted from `src/libstd/sys/windows/os.rs` in the Rust distribution.

// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use libc::{self, DWORD};
use kernel32;

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Debug)]
pub struct Errno(pub DWORD);

impl fmt::Display for Errno {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Display an actual message
        write!(fmt, "Error {}", self.0)
    }
}

pub fn errno() -> Errno {
    unsafe {
        Errno(kernel32::GetLastError())
    }
}

pub fn set_errno(Errno(errno): Errno) {
    unsafe {
        kernel32::SetLastError(errno)
    }
}
