[![Crates.io](https://img.shields.io/crates/v/const-c-str.svg)](https://crates.io/crates/const-c-str)
[![api](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/const-c-str/0.1.0/const_c_str/)

# const-c-str

Safely create &CStr at compile time checked with [from_bytes_with_nul](https://doc.rust-lang.org/std/ffi/struct.CStr.html#method.from_bytes_with_nul)
## Examples
```rust
use const_c_str::c_str;

#[cfg(feature = "const_cstr_unchecked")]
const greeting: &std::ffi::CStr = c_str!("Hello World!");
```
