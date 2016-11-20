# errno [![Build status](https://img.shields.io/travis/lfairy/rust-errno.svg)](http://travis-ci.org/lfairy/rust-errno) [![Build status](https://ci.appveyor.com/api/projects/status/0fgngg808u7xwto8?svg=true)](https://ci.appveyor.com/project/lfairy/rust-errno) [![Cargo](https://img.shields.io/crates/v/errno.svg)](https://crates.io/crates/errno)

Cross-platform interface to the [`errno`][errno] variable.

Documentation is available at <https://docs.rs/errno>.

[errno]: https://en.wikipedia.org/wiki/Errno.h


## Dependency

Add to your `Cargo.toml`:

```toml
[dependencies]
errno = "*"
libc = "*"
```


## Examples

```rust
extern crate errno;
use errno::{Errno, errno, set_errno};

// Get the current value of errno
let e = errno();

// Set the current value of errno
set_errno(e);

// Extract the error code as an i32
let code = e.0;

// Display a human-friendly error message
println!("Error {}: {}", code, e);
```
