extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;

use quote::quote;
use syn::parse_macro_input;

#[proc_macro_hack]
pub fn c_str(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::LitStr);

    let mut bytes = input.value().into_bytes();
    bytes.push(0);

    TokenStream::from(
        if let Some(_) = std::ffi::CStr::from_bytes_with_nul(&bytes).err() {
            syn::Error::new_spanned(input, "The string contains interior nul byte(s).")
                .to_compile_error()
        } else {
            quote! {
                unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(&[#(#bytes,)*]) }
            }
        },
    )
}
