/*!
# Bitmask-Enum
A bitmask can have unsigned integer types, the default type is `usize`.

To see a better documentation run `cargo doc --open` and select your `Bitmask`.

```ignore
#[bitmask] // usize
enum Bitmask { /* ... */ }

#[bitmask(u8)] // u8
enum Bitmask { /* ... */ }
```

## Implemented Methods
```ignore
// contains all values
const fn all() -> Self;

// self contains all values
const fn is_all(&self) -> bool;

// contains no value
const fn none() -> Self;

// self contains no value
const fn is_none(&self) -> bool;

// self contains one of the other
// (self & other) != 0 || other == 0
const fn contains(&self, other: Self) -> bool;

// self contains all of the other
// (self & other) == other
const fn contains_all(&self, other: Self) -> bool;

// constant bitwise ops
const fn not(self) -> Self;
const fn and(self, other: Self) -> Self;
const fn or(self, other: Self) -> Self;
const fn xor(self, other: Self) -> Self;
```

## Implemented Traits
```ignore
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

impl std::ops::Not;

impl std::ops::BitAnd;
impl std::ops::BitAndAssign;

impl std::ops::BitOr;
impl std::ops::BitOrAssign;

impl std::ops::BitXor;
impl std::ops::BitXorAssign;

impl From<#type>;
impl Into<#type>;

impl PartialEq<#type>;

impl std::fmt::Binary;
```
*/

use proc_macro::TokenStream;
use syn::{parse_macro_input, Ident, ItemEnum};

#[proc_macro_attribute]
pub fn bitmask(attr: TokenStream, item: TokenStream) -> TokenStream {
    let typ = quote::format_ident!("{}", typ(attr));

    let item = parse_macro_input!(item as ItemEnum);
    let vis = item.vis.clone();
    let (ident, idents, exprs) = enm(item);

    let enm = quote::quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #vis struct #ident(#typ);

        #[allow(non_upper_case_globals)]
        impl #ident {
            #[doc(hidden)]
            const __CONST_TWO: #typ = 2;

            #(#vis const #idents: #ident = #exprs;)*

            /// contains all values
            #[inline]
            #vis const fn all() -> Self {
                Self(!0)
            }

            /// self contains all values
            #[inline]
            #vis const fn is_all(&self) -> bool {
                self.0 == !0
            }

            /// contains no value
            #[inline]
            #vis const fn none() -> Self {
                Self(0)
            }

            /// self contains no value
            #[inline]
            #vis const fn is_none(&self) -> bool {
                self.0 == 0
            }

            /// self contains one of the other
            /// `(self & other) != 0 || other == 0`
            #[inline]
            #vis const fn contains(&self, other: Self) -> bool {
                (self.0 & other.0) != 0 || other.0 == 0
            }

            /// self contains all of the other
            /// `(self & other) == other`
            #[inline]
            #vis const fn contains_all(&self, other: Self) -> bool {
                (self.0 & other.0) == other.0
            }

            /// constant bitwise not
            #[inline]
            #vis const fn not(self) -> Self {
                Self(!self.0)
            }

            /// constant bitwise and
            #[inline]
            #vis const fn and(self, other: Self) -> Self {
                Self(self.0 & other.0)
            }

            /// constant bitwise or
            #[inline]
            #vis const fn or(self, other: Self) -> Self {
                Self(self.0 | other.0)
            }

            /// constant bitwise xor
            #[inline]
            #vis const fn xor(self, other: Self) -> Self {
                Self(self.0 ^ other.0)
            }
        }

        impl std::ops::Not for #ident {
            type Output = Self;
            fn not(self) -> Self::Output {
                Self(self.0.not())
            }
        }

        impl std::ops::BitAnd for #ident {
            type Output = Self;
            fn bitand(self, rhs: Self) -> Self::Output {
                Self(self.0.bitand(rhs.0))
            }
        }

        impl std::ops::BitAndAssign for #ident {
            fn bitand_assign(&mut self, rhs: Self){
                self.0.bitand_assign(rhs.0)
            }
        }

        impl std::ops::BitOr for #ident {
            type Output = Self;
            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0.bitor(rhs.0))
            }
        }

        impl std::ops::BitOrAssign for #ident {
            fn bitor_assign(&mut self, rhs: Self){
                self.0.bitor_assign(rhs.0)
            }
        }

        impl std::ops::BitXor for #ident {
            type Output = Self;
            fn bitxor(self, rhs: Self) -> Self::Output {
                Self(self.0.bitxor(rhs.0))
            }
        }

        impl std::ops::BitXorAssign for #ident {
            fn bitxor_assign(&mut self, rhs: Self){
                self.0.bitxor_assign(rhs.0)
            }
        }

        impl From<#typ> for #ident {
            fn from(val: #typ) -> Self {
                Self(val)
            }
        }

        impl Into<#typ> for #ident {
            fn into(self) -> #typ {
                self.0
            }
        }

        impl PartialEq<#typ> for #ident {
            fn eq(&self, other: &#typ) -> bool {
                self.0 == *other
            }
        }

        impl std::fmt::Binary for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };

    TokenStream::from(enm)
}

fn typ(attr: TokenStream) -> String {
    if attr.is_empty() {
        "usize".to_owned()
    } else {
        let typ = attr.into_iter().next().expect("unreachable").to_string();
        match typ.as_str() {
            "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => (),
            _ => panic!("type can only be an unsigned integer."),
        }
        typ
    }
}

fn enm(item: ItemEnum) -> (Ident, Vec<Ident>, Vec<impl quote::ToTokens>) {
    let ident = item.ident;

    let mut has_vals: Option<bool> = None;

    let mut idents = Vec::with_capacity(item.variants.len());
    let mut exprs = Vec::with_capacity(item.variants.len());
    item.variants.iter().enumerate().for_each(|(i, v)| {
        idents.push(v.ident.clone());

        let hv = has_vals.unwrap_or_else(|| {
            let hv = v.discriminant.is_some();
            has_vals.replace(hv);
            hv
        });

        if hv != v.discriminant.is_some() {
            panic!("the bitmask can either have assigned or default values, not both.");
        }

        if hv {
            if let Some((_, ref expr)) = v.discriminant.as_ref() {
                match expr {
                    syn::Expr::Lit(ref lit) => exprs.push(quote::quote!(#ident(#lit))),
                    _ => exprs.push(quote::quote!(#expr)),
                }
            } else {
                panic!("the bitmask can either have assigned or default values, not both.");
            }
        } else {
            exprs.push(quote::quote!(#ident(Self::__CONST_TWO.pow(#i as u32))));
        }
    });

    (ident, idents, exprs)
}
