/// Wrappers around `errno`.

extern crate libc;

#[cfg(unix)] mod unix;
#[cfg(unix)] pub use unix::{Errno, errno, set_errno};

#[cfg(windows)] mod windows;
#[cfg(windows)] pub use windows::{Errno, errno, set_errno};

#[test]
fn it_works() {
    let x = errno();
    set_errno(x);
    let _ = x.to_string();
}
