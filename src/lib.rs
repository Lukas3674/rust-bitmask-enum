/*!
# Bitmask-Enum

A bitmask enum attribute macro, to turn an enum into a bitmask.

A bitmask can have unsigned integer types, the default type is `usize`.

```
use bitmask_enum::bitmask;

#[bitmask] // usize
enum Bitmask { /* ... */ }

#[bitmask(u8)] // u8
enum BitmaskU8 { /* ... */ }
```

## Example

```
use bitmask_enum::bitmask;

#[bitmask(u8)]
enum Bitmask {
    Flag1, // defaults to 0d00000001
    Flag2, // defaults to 0d00000010
    Flag3, // defaults to 0d00000100
}

// bitmask has const bitwise operator methods
const CONST_BM: Bitmask = Bitmask::Flag2.or(Bitmask::Flag3);

println!("{:#010b}", CONST_BM); // 0b00000110

// Bitmask that contains Flag1 and Flag3
let bm = Bitmask::Flag1 | Bitmask::Flag3;

println!("{:#010b}", bm); // 0b00000101

// Does bm intersect one of CONST_BM
println!("{}", bm.intersects(CONST_BM)); // true

// Does bm contain all of CONST_BM
println!("{}", bm.contains(CONST_BM)); // false
```

## Custom Values

You can assign every flag a custom value.

Because behind the scences `enum Bitmask` gets converted to a `struct Bitmask(u8);` you need to wrap `u8` expressions into a `Self(_)`.

```
use bitmask_enum::bitmask;

#[bitmask(u8)]
enum Bitmask {
    Flag1 = Self(0b00010000),
    Flag2 = Self(0b00000100),
    Flag3 = Self(0b00000001),

    Flag13_1 = Self(0b00010000 | 0b00000001),
    Flag13_2 = Self::Flag1.or(Self::Flag3),

    Flag4 = Self({
        let left = Self::Flag13_1;
        left.0 | Self::Flag2.0
    }),
}

let bm = Bitmask::Flag1 | Bitmask::Flag3;

println!("{:#010b}", bm); // 0b00010001
println!("{}", bm == Bitmask::Flag13_1); // true
println!("{}", bm == Bitmask::Flag13_2); // true

println!("{:#010b}", Bitmask::Flag4); // 0b00010101
```

## Implemented Methods
```ignore
// contains all values
const fn all() -> Self;

// if self contains all values
const fn is_all(&self) -> bool;

// contains no value
const fn none() -> Self;

// if self contains no value
const fn is_none(&self) -> bool;

// self intersects one of the other
// (self & other) != 0 || other == 0
const fn intersects(&self, other: Self) -> bool;

// self contains all of the other
// (self & other) == other
const fn contains(&self, other: Self) -> bool;

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

impl From<#type> for #ident;
impl From<#ident> for #type;

impl PartialEq<#type>;

impl std::fmt::Binary;
```
*/

use proc_macro::TokenStream;
use syn::{parse_macro_input, Ident, ItemEnum};

/**
# Bitmask-Enum

A bitmask can have unsigned integer types, the default type is `usize`.

```
use bitmask_enum::bitmask;

#[bitmask] // usize
enum Bitmask { /* ... */ }

#[bitmask(u8)] // u8
enum BitmaskU8 { /* ... */ }
```
*/
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
            #(#vis const #idents: #ident = #exprs;)*

            /// contains all values
            #[inline]
            #vis const fn all() -> Self {
                Self(!0)
            }

            /// if self contains all values
            #[inline]
            #vis const fn is_all(&self) -> bool {
                self.0 == !0
            }

            /// contains no value
            #[inline]
            #vis const fn none() -> Self {
                Self(0)
            }

            /// if self contains no value
            #[inline]
            #vis const fn is_none(&self) -> bool {
                self.0 == 0
            }

            /// self intersects one of the other
            /// `(self & other) != 0 || other == 0`
            #[inline]
            #vis const fn intersects(&self, other: Self) -> bool {
                (self.0 & other.0) != 0 || other.0 == 0
            }

            /// self contains all of the other
            /// `(self & other) == other`
            #[inline]
            #vis const fn contains(&self, other: Self) -> bool {
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
            #[inline]
            fn not(self) -> Self::Output {
                Self(self.0.not())
            }
        }

        impl std::ops::BitAnd for #ident {
            type Output = Self;
            #[inline]
            fn bitand(self, rhs: Self) -> Self::Output {
                Self(self.0.bitand(rhs.0))
            }
        }

        impl std::ops::BitAndAssign for #ident {
            #[inline]
            fn bitand_assign(&mut self, rhs: Self){
                self.0.bitand_assign(rhs.0)
            }
        }

        impl std::ops::BitOr for #ident {
            type Output = Self;
            #[inline]
            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0.bitor(rhs.0))
            }
        }

        impl std::ops::BitOrAssign for #ident {
            #[inline]
            fn bitor_assign(&mut self, rhs: Self){
                self.0.bitor_assign(rhs.0)
            }
        }

        impl std::ops::BitXor for #ident {
            type Output = Self;
            #[inline]
            fn bitxor(self, rhs: Self) -> Self::Output {
                Self(self.0.bitxor(rhs.0))
            }
        }

        impl std::ops::BitXorAssign for #ident {
            #[inline]
            fn bitxor_assign(&mut self, rhs: Self){
                self.0.bitxor_assign(rhs.0)
            }
        }

        impl From<#typ> for #ident {
            #[inline]
            fn from(val: #typ) -> Self {
                Self(val)
            }
        }

        impl From<#ident> for #typ {
            #[inline]
            fn from(val: #ident) -> #typ {
                val.0
            }
        }

        impl PartialEq<#typ> for #ident {
            #[inline]
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

        assert!(
            hv == v.discriminant.is_some(),
            "the bitmask can either have assigned or default values, not both."
        );

        if hv {
            let (_, ref expr) = v.discriminant.as_ref().expect("unreachable");
            exprs.push(quote::quote!(#expr));
        } else {
            exprs.push(quote::quote!(#ident(1 << #i)));
        }
    });

    (ident, idents, exprs)
}
