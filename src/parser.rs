use proc_macro::{Span, TokenStream};
use syn::{
    parse::{Parse, ParseStream},
    Ident, ItemEnum, Token,
};

pub fn parse(attr: TokenStream, item: ItemEnum) -> TokenStream {
    let args = parse_args(attr);
    let typ = args.typ;

    let vis = item.vis;
    let attr = item.attrs;
    let ident = item.ident;

    let has_vals: bool = item
        .variants
        .first()
        .map_or(false, |v| v.discriminant.is_some());

    let flag_consts = item.variants.iter().enumerate().map(|(i, v)| {
        if has_vals != v.discriminant.is_some() {
            panic!("the bitmask can either have assigned or default values, not both.")
        }
        let variant_attrs = &v.attrs;
        let variant_ident = &v.ident;
        let expr = if has_vals {
            let (_, ref expr) = v.discriminant.as_ref().expect("unreachable");
            quote::quote!(Self { bits: #expr })
        } else {
            quote::quote!(Self { bits: 1 << #i })
        };

        let mut out = quote::quote!(
            #(#variant_attrs)*
            #vis const #variant_ident: #ident = #expr;
        );
        if args.inverted_flags {
            let inverted_variant_ident = Ident::new(&format!("Inverted{}", v.ident), ident.span());
            let inverted_expr = if has_vals {
                let (_, ref expr) = v.discriminant.as_ref().expect("unreachable");
                quote::quote!(Self { bits: #expr ^ !0 })
            } else {
                quote::quote!(Self { bits: (1 << #i) ^ !0 })
            };
            out = quote::quote!(
                #out
                #(#variant_attrs)*
                #vis const #inverted_variant_ident: #ident = #inverted_expr;
            );
        }
        out
    });

    TokenStream::from(quote::quote! {
        #(#attr)*
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #vis struct #ident {
            bits: #typ,
        }

        #[allow(non_upper_case_globals)]
        impl #ident {
            #(#flag_consts)*

            /// returns the underlying bits
            #[inline]
            #vis const fn bits(&self) -> #typ {
                self.bits
            }

            /// contains all values
            #[inline]
            #vis const fn all() -> Self {
                Self { bits: !0 }
            }

            /// if self contains all values
            #[inline]
            #vis const fn is_all(&self) -> bool {
                self.bits == !0
            }

            /// contains no value
            #[inline]
            #vis const fn none() -> Self {
                Self { bits: 0 }
            }

            /// if self contains no value
            #[inline]
            #vis const fn is_none(&self) -> bool {
                self.bits == 0
            }

            /// self intersects one of the other
            /// `(self & other) != 0 || other == 0`
            #[inline]
            #vis const fn intersects(&self, other: Self) -> bool {
                (self.bits & other.bits) != 0 || other.bits == 0
            }

            /// self contains all of the other
            /// `(self & other) == other`
            #[inline]
            #vis const fn contains(&self, other: Self) -> bool {
                (self.bits & other.bits) == other.bits
            }

            /// constant bitwise not
            #[inline]
            #vis const fn not(self) -> Self {
                Self { bits: !self.bits }
            }

            /// constant bitwise and
            #[inline]
            #vis const fn and(self, other: Self) -> Self {
                Self { bits: self.bits & other.bits }
            }

            /// constant bitwise or
            #[inline]
            #vis const fn or(self, other: Self) -> Self {
                Self { bits: self.bits | other.bits }
            }

            /// constant bitwise xor
            #[inline]
            #vis const fn xor(self, other: Self) -> Self {
                Self { bits: self.bits ^ other.bits }
            }
        }

        impl core::ops::Not for #ident {
            type Output = Self;
            #[inline]
            fn not(self) -> Self::Output {
                Self { bits: self.bits.not() }
            }
        }

        impl core::ops::BitAnd for #ident {
            type Output = Self;
            #[inline]
            fn bitand(self, rhs: Self) -> Self::Output {
                Self { bits: self.bits.bitand(rhs.bits) }
            }
        }

        impl core::ops::BitAndAssign for #ident {
            #[inline]
            fn bitand_assign(&mut self, rhs: Self){
                self.bits.bitand_assign(rhs.bits)
            }
        }

        impl core::ops::BitOr for #ident {
            type Output = Self;
            #[inline]
            fn bitor(self, rhs: Self) -> Self::Output {
                Self { bits: self.bits.bitor(rhs.bits) }
            }
        }

        impl core::ops::BitOrAssign for #ident {
            #[inline]
            fn bitor_assign(&mut self, rhs: Self){
                self.bits.bitor_assign(rhs.bits)
            }
        }

        impl core::ops::BitXor for #ident {
            type Output = Self;
            #[inline]
            fn bitxor(self, rhs: Self) -> Self::Output {
                Self { bits: self.bits.bitxor(rhs.bits) }
            }
        }

        impl core::ops::BitXorAssign for #ident {
            #[inline]
            fn bitxor_assign(&mut self, rhs: Self){
                self.bits.bitxor_assign(rhs.bits)
            }
        }

        impl From<#typ> for #ident {
            #[inline]
            fn from(val: #typ) -> Self {
                Self { bits: val }
            }
        }

        impl From<#ident> for #typ {
            #[inline]
            fn from(val: #ident) -> #typ {
                val.bits
            }
        }

        impl PartialEq<#typ> for #ident {
            #[inline]
            fn eq(&self, other: &#typ) -> bool {
                self.bits == *other
            }
        }

        impl core::fmt::Binary for #ident {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.bits.fmt(f)
            }
        }

        impl core::fmt::LowerHex for #ident {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.bits.fmt(f)
            }
        }

        impl core::fmt::UpperHex for #ident {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.bits.fmt(f)
            }
        }

        impl core::fmt::Octal for #ident {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.bits.fmt(f)
            }
        }
    })
}

struct BitmaskArgs {
    pub typ: Ident,
    pub inverted_flags: bool,
}

impl BitmaskArgs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn typ(mut self, typ: Ident) -> Self {
        self.typ = typ;
        self
    }

    pub fn inverted_flags(mut self, inverted_flags: bool) -> Self {
        self.inverted_flags = inverted_flags;
        self
    }
}

impl Default for BitmaskArgs {
    fn default() -> Self {
        Self {
            typ: Ident::new("usize", Span::call_site().into()),
            inverted_flags: false,
        }
    }
}

mod kw {
    syn::custom_keyword!(inverted_flags);
}

impl Parse for BitmaskArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut bitmask_args = Self::new();
        loop {
            if input.is_empty() {
                break;
            }

            let lookahead = input.lookahead1();
            if lookahead.peek(kw::inverted_flags) {
                input.parse::<kw::inverted_flags>()?;
                bitmask_args = bitmask_args.inverted_flags(true);
            } else if lookahead.peek(Ident) {
                let ident = input.parse::<Ident>()?;
                match ident.to_string().as_str() {
                    #[rustfmt::skip]
                    "u8" | "u16" | "u32" | "u64" | "u128" | "usize" |
                    "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => {
                        bitmask_args = bitmask_args.typ(ident);
                    }
                    _ => panic!("type can only be an (un)signed integer."),
                }
            }

            if input.is_empty() {
                break;
            }

            input.parse::<Token!(,)>()?;
        }
        Ok(bitmask_args)
    }
}

fn parse_args(args: TokenStream) -> BitmaskArgs {
    let bitmask_args = match syn::parse::<BitmaskArgs>(args) {
        Ok(ok) => ok,
        Err(err) => {
            panic!("Could not parse attribute: {}", err);
        }
    };
    bitmask_args
}
