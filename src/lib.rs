#![doc = include_str!("../README.md")]

mod parser;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemEnum};

/// ## Bitmask-Enum
///
/// A bitmask can have unsigned integer types, the default type is `usize`.
///
/// ```
/// use bitmask_enum::bitmask;
///
/// #[bitmask] // usize
/// enum Bitmask { /* ... */ }
///
/// #[bitmask(u8)] // u8
/// enum BitmaskU8 { /* ... */ }
/// ```
#[proc_macro_attribute]
pub fn bitmask(attr: TokenStream, item: TokenStream) -> TokenStream {
    match parser::parse(attr, parse_macro_input!(item as ItemEnum)) {
        Ok(ts) => ts,
        Err(err) => err.into_compile_error().into(),
    }
}
