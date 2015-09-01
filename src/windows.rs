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
use std::ptr;
use kernel32;
use winapi::{DWORD, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS, WCHAR};

#[derive(Copy, Clone, Eq, Ord, PartialEq, PartialOrd, Debug)]
pub struct Errno(pub DWORD);

impl fmt::Display for Errno {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        // This value is calculated from the macro
        // MAKELANGID(LANG_SYSTEM_DEFAULT, SUBLANG_SYS_DEFAULT)
        let lang_id = 0x0800 as DWORD;

        let mut buf = [0 as WCHAR; 2048];

        unsafe {
            let res = kernel32::FormatMessageW(FORMAT_MESSAGE_FROM_SYSTEM |
                                               FORMAT_MESSAGE_IGNORE_INSERTS,
                                               ptr::null_mut(),
                                               self.0,
                                               lang_id,
                                               buf.as_mut_ptr(),
                                               buf.len() as DWORD,
                                               ptr::null_mut());
            if res == 0 {
                // Sometimes FormatMessageW can fail e.g. system doesn't like langId
                let Errno(fm_err) = errno();
                return write!(fmt,
                              "OS Error {} (FormatMessageW returned error {})",
                              self.0, fm_err);
            }

            let end = buf.iter().position(|&i| i == 0).unwrap_or(buf.len());
            // Can we skip this allocation?
            let msg = String::from_utf16(&buf[..end]);
            match msg {
                Ok(msg) => fmt.write_str(&msg),
                Err(..) =>
                    write!(fmt,
                           "OS Error {} (FormatMessageW returned invalid UTF-16)",
                           self.0),
            }
        }
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
