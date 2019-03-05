//! Safely create &CStr at compile time checked with [from_bytes_with_nul](std::ffi::CStr::from_bytes_with_nul)
//! # Examples
//! ```
//! use const_c_str::c_str;
//!
//! #[cfg(feature = "const_cstr_unchecked")]
//! const greeting: &std::ffi::CStr = c_str!("Hello World!");
//! ```
use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
pub use const_c_str_impl::c_str;

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "const_cstr_unchecked")]
    const greeting: &std::ffi::CStr = c_str!("Hello World!");

    #[test]
    fn test() {
        println!("{:?}", c_str!("Testing!"));
    }
}
