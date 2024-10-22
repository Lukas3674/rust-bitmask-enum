use proc_macro::{Span, TokenStream};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Error, Ident, ItemEnum, Result, Token,
};

pub fn parse(attr: TokenStream, mut item: ItemEnum) -> Result<TokenStream> {
    let typ = parse_typ(attr)?;

    let config = if let Some(idx) = item
        .attrs
        .iter()
        .enumerate()
        .find_map(|(idx, attr)| attr.path().is_ident("bitmask_config").then_some(idx))
    {
        item.attrs.remove(idx).parse_args::<Config>()?
    } else {
        Config::new()
    };

    let vis = item.vis;
    let attrs = item.attrs;
    let ident = item.ident;

    let mut flags_amount = item.variants.len();

    if config.inverted_flags {
        flags_amount *= 2;
    }

    let mut all_flags = Vec::with_capacity(flags_amount);
    let mut all_flags_names = Vec::with_capacity(flags_amount);

    let mut i: usize = 0;
    let mut flags = Vec::with_capacity(flags_amount);
    for v in item.variants.iter() {
        let v_attrs = &v.attrs;
        let v_ident = &v.ident;

        all_flags.push(v_ident.clone());
        all_flags_names.push(quote::quote!(stringify!(#v_ident)));

        let expr = if let Some((_, expr)) = v.discriminant.as_ref() {
            quote::quote!(#expr)
        } else {
            let expr = quote::quote!(1 << #i);
            i += 1;
            expr
        };

        let i_flag = config
            .inverted_flags
            .then(|| {
                let i_ident = Ident::new(&format!("Inverted{}", v_ident), v_ident.span());

                all_flags.push(i_ident.clone());
                all_flags_names.push(quote::quote!(stringify!(#i_ident)));

                quote::quote!(
                    #(#v_attrs)*
                    #vis const #i_ident: #ident = Self { bits: (#expr) ^ !0 };
                )
            })
            .into_iter();

        flags.push(quote::quote!(
            #(#v_attrs)*
            #vis const #v_ident: #ident = Self { bits: #expr };

            #(#i_flag)*
        ))
    }

    let flags_iter = config.flags_iter.then(|| {
        quote::quote!(
            /// Returns an iterator over all flags of the bitmask.
            /// Where each Item = (name, flag).
            #vis fn flags() -> impl core::iter::Iterator<Item = &'static (&'static str, Self)> {
                static FLAGS: [(&'static str, #ident); #flags_amount] = [
                    #((#all_flags_names, #ident::#all_flags),)*
                ];

                FLAGS.iter()
            }
        )
    }).into_iter();

    let debug_impl = if config.vec_debug {
        quote::quote! {
            impl core::fmt::Debug for #ident {
                fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                    write!(f, "{}[", stringify!(#ident))?;

                    let mut has_flags = false;
                    #(if self.contains(Self::#all_flags) {
                        if has_flags {
                            write!(f, ", {}", #all_flags_names)?;
                        } else {
                            write!(f, "{}", #all_flags_names)?;
                            has_flags = true;
                        }
                    })*

                    write!(f, "]")
                }
            }
        }
    } else {
        quote::quote! {
            impl core::fmt::Debug for #ident {
                fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                    f.debug_struct(stringify!(#ident))
                        .field("bits", &self.bits)
                        .finish()
                }
            }
        }
    };

    Ok(TokenStream::from(quote::quote! {
        #(#attrs)*
        #[repr(transparent)]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #vis struct #ident {
            bits: #typ,
        }

        #[allow(non_upper_case_globals)]
        impl #ident {
            #(#flags)*

            #(#flags_iter)*

            /// Returns the underlying bits of the bitmask.
            #[inline]
            #vis const fn bits(&self) -> #typ {
                self.bits
            }

            /// Returns a bitmask that contains all values.
            ///
            /// This will include bits that do not have any flags.
            /// Use `::all_flags()` if you only want to use flags.
            #[inline]
            #vis const fn all_bits() -> Self {
                Self { bits: !0 }
            }

            /// Returns a bitmask that contains all flags.
            #[inline]
            #vis const fn all_flags() -> Self {
                Self { bits: #(Self::#all_flags.bits |)* 0 }
            }

            /// Returns `true` if the bitmask contains all values.
            ///
            /// This will check for `bits == !0`,
            /// use `.is_all_flags()` if you only want to check for all flags
            #[inline]
            #vis const fn is_all_bits(&self) -> bool {
                self.bits == !0
            }

            /// Returns `true` if the bitmask contains all flags.
            ///
            /// This will fail if any unused bit is set,
            /// consider using `.truncate()` first.
            #[inline]
            #vis const fn is_all_flags(&self) -> bool {
                self.bits == Self::all_flags().bits
            }

            /// Returns a bitmask that contains all values.
            ///
            /// This will include bits that do not have any flags.
            /// Use `::all_flags()` if you only want to use flags.
            #[inline]
            #[deprecated(note = "Please use the `::all_bits()` constructor")]
            #vis const fn all() -> Self {
                Self::all_bits()
            }

            /// Returns `true` if the bitmask contains all values.
            ///
            /// This will check for `bits == !0`,
            /// use `.is_all_flags()` if you only want to check for all flags
            #[inline]
            #[deprecated(note = "Please use the `.is_all_bits()` method")]
            #vis const fn is_all(&self) -> bool {
                self.is_all_bits()
            }


            /// Returns a bitmask that contains all flags.
            #[inline]
            #[deprecated(note = "Please use the `::all_flags()` constructor")]
            #vis const fn full() -> Self {
                Self::all_flags()
            }

            /// Returns `true` if the bitmask contains all flags.
            ///
            /// This will fail if any unused bit is set,
            /// consider using `.truncate()` first.
            #[inline]
            #[deprecated(note = "Please use the `.is_all_flags()` method")]
            #vis const fn is_full(&self) -> bool {
                self.is_all_flags()
            }

            /// Returns a bitmask that does not contain any values.
            #[inline]
            #vis const fn none() -> Self {
                Self { bits: 0 }
            }

            /// Returns `true` if the bitmask does not contain any values.
            #[inline]
            #vis const fn is_none(&self) -> bool {
                self.bits == 0
            }

            /// Returns a bitmask that only has bits corresponding to flags
            #[inline]
            #vis const fn truncate(&self) -> Self {
                Self { bits: self.bits & Self::all_flags().bits }
            }

            /// Returns `true` if `self` intersects with any value in `other`,
            /// or if `other` does not contain any values.
            ///
            /// This is equivalent to `(self & other) != 0 || other == 0`.
            #[inline]
            #vis const fn intersects(&self, other: Self) -> bool {
                (self.bits & other.bits) != 0 || other.bits == 0
            }

            /// Returns `true` if `self` contains all values of `other`.
            ///
            /// This is equivalent to  `(self & other) == other`.
            #[inline]
            #vis const fn contains(&self, other: Self) -> bool {
                (self.bits & other.bits) == other.bits
            }

            /// Returns the bitwise NOT of the bitmask.
            #[inline]
            #vis const fn not(self) -> Self {
                Self { bits: !self.bits }
            }

            /// Returns the bitwise AND of the bitmask.
            #[inline]
            #vis const fn and(self, other: Self) -> Self {
                Self { bits: self.bits & other.bits }
            }

            /// Returns the bitwise OR of the bitmask.
            #[inline]
            #vis const fn or(self, other: Self) -> Self {
                Self { bits: self.bits | other.bits }
            }

            /// Returns the bitwise XOR of the bitmask.
            #[inline]
            #vis const fn xor(self, other: Self) -> Self {
                Self { bits: self.bits ^ other.bits }
            }
        }

        impl core::ops::Not for #ident {
            type Output = Self;
            #[inline]
            fn not(self) -> Self::Output {
                Self { bits: core::ops::Not::not(self.bits) }
            }
        }

        impl core::ops::BitAnd for #ident {
            type Output = Self;
            #[inline]
            fn bitand(self, rhs: Self) -> Self::Output {
                Self { bits: core::ops::BitAnd::bitand(self.bits, rhs.bits) }
            }
        }

        impl core::ops::BitAndAssign for #ident {
            #[inline]
            fn bitand_assign(&mut self, rhs: Self) {
                core::ops::BitAndAssign::bitand_assign(&mut self.bits, rhs.bits)
            }
        }

        impl core::ops::BitOr for #ident {
            type Output = Self;
            #[inline]
            fn bitor(self, rhs: Self) -> Self::Output {
                Self { bits: core::ops::BitOr::bitor(self.bits, rhs.bits) }
            }
        }

        impl core::ops::BitOrAssign for #ident {
            #[inline]
            fn bitor_assign(&mut self, rhs: Self) {
                core::ops::BitOrAssign::bitor_assign(&mut self.bits, rhs.bits)
            }
        }

        impl core::ops::BitXor for #ident {
            type Output = Self;
            #[inline]
            fn bitxor(self, rhs: Self) -> Self::Output {
                Self { bits: core::ops::BitXor::bitxor(self.bits, rhs.bits) }
            }
        }

        impl core::ops::BitXorAssign for #ident {
            #[inline]
            fn bitxor_assign(&mut self, rhs: Self) {
                core::ops::BitXorAssign::bitxor_assign(&mut self.bits, rhs.bits)
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

        #debug_impl

        impl core::fmt::Binary for #ident {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Binary::fmt(&self.bits, f)
            }
        }

        impl core::fmt::LowerHex for #ident {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::LowerHex::fmt(&self.bits, f)
            }
        }

        impl core::fmt::UpperHex for #ident {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::UpperHex::fmt(&self.bits, f)
            }
        }

        impl core::fmt::Octal for #ident {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Octal::fmt(&self.bits, f)
            }
        }
    }))
}

fn parse_typ(attr: TokenStream) -> Result<Ident> {
    if attr.is_empty() {
        Ok(Ident::new("usize", Span::call_site().into()))
    } else {
        let ident = syn::parse::<Ident>(attr)?;
        match ident.to_string().as_str() {
            #[rustfmt::skip]
            "u8" | "u16" | "u32" | "u64" | "u128" | "usize" |
            "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => Ok(ident),
            _ => Err(Error::new_spanned(
                ident,
                "type can only be an (un)signed integer",
            )),
        }
    }
}

struct Config {
    inverted_flags: bool,
    vec_debug: bool,
    flags_iter: bool,
}

impl Config {
    fn new() -> Self {
        Self {
            inverted_flags: false,
            vec_debug: false,
            flags_iter: false,
        }
    }
}

impl Parse for Config {
    fn parse(input: ParseStream) -> Result<Self> {
        let args = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        let mut config = Self::new();
        for arg in args {
            match arg.to_string().as_str() {
                "inverted_flags" => config.inverted_flags = true,
                "vec_debug" => config.vec_debug = true,
                "flags_iter" => config.flags_iter = true,
                _ => return Err(Error::new_spanned(arg, "unknown config option")),
            }
        }
        Ok(config)
    }
}
