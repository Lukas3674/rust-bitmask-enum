# Bitmask-Enum

[![API](https://docs.rs/bitmask-enum/badge.svg)](https://docs.rs/bitmask-enum) [![Crate](https://img.shields.io/crates/v/bitmask-enum.svg)](https://crates.io/crates/bitmask-enum)

A bitmask enum attribute macro, to turn an enum into a bitmask.

A bitmask can have (un)signed integer types, the default type is `usize`.

First created because I wanted something simple, evolved with inspiration from
the [bitflags](https://crates.io/crates/bitflags) crate, which might be something
you want to take a look at.

```rust
use bitmask_enum::bitmask;

#[bitmask] // usize
enum Bitmask { /* ... */ }

#[bitmask(u8)] // u8
enum BitmaskU8 { /* ... */ }
```

## Example

```rust
use bitmask_enum::bitmask;

#[bitmask(u8)]
enum Bitmask {
    Flag1, // defaults to 0b00000001
    Flag2, // defaults to 0b00000010
    Flag3, // defaults to 0b00000100
}

// It is possible to impl on the bitmask and use its bits field
impl Bitmask {
    fn _set_to(&mut self, val: u8) {
        self.bits = val
    }
}

// bitmask has const bitwise operator methods
const CONST_BM: Bitmask = Bitmask::Flag2.or(Bitmask::Flag3);

fn main() {
    println!("{:#010b}", CONST_BM); // 0b00000110

    // Bitmask that contains Flag1 and Flag3
    let bm = Bitmask::Flag1 | Bitmask::Flag3;

    println!("{:#010b}", bm); // 0b00000101

    // Does bm intersect one of CONST_BM
    println!("{}", bm.intersects(CONST_BM)); // true

    // Does bm contain all of CONST_BM
    println!("{}", bm.contains(CONST_BM)); // false
}
```

## Custom Values

You can assign any flag a custom value.

```rust
use bitmask_enum::bitmask;

#[bitmask(u8)]
enum Bitmask {
    Flag1, // defaults to 0b00000001

    CustomFlag3 = 0b00000100,

    Flag2, // defaults to 0b00000010
    Flag3, // defaults to 0b00000100

    Flag13_1 = 0b00000001 | 0b00000100,
    Flag13_2 = Self::Flag1.or(Self::Flag3).bits,
    Flag13_3 = Self::Flag1.bits | Self::CustomFlag3.bits,

    Flag123 = {
        let flag13 = Self::Flag13_1.bits;
        flag13 | Self::Flag2.bits
    },
}

fn main() {
    let bm = Bitmask::Flag1 | Bitmask::Flag3;

    println!("{:#010b}", bm); // 0b00000101
    println!("{}", bm == Bitmask::Flag13_1); // true

    println!("{:#010b}", Bitmask::Flag123); // 0b00000111
}
```

## Bitmask Config

It is possible to add custom bitmask config options via the `#[bitmask_config(...)]` macro. (Just add it below the `#[bitmask]` macro)

```rust
use bitmask_enum::bitmask;

#[bitmask(u8)]
#[bitmask_config(inverted_flags)]
enum Bitmask {
    Flag1, // defaults to 0b00000001
}

#[bitmask(u8)]
#[bitmask_config(vec_debug)]
enum BitmaskVecDebug {
    Flag1,
    Flag2,
}

#[bitmask(u8)]
#[bitmask_config(flags_iter)]
enum BitmaskFlagsIter {
    Flag1,
    Flag2,
}

fn main() {
    println!("{:#010b}", Bitmask::Flag1); // 0b00000001
    println!("{:#010b}", Bitmask::InvertedFlag1); // 0b11111110

    println!("{:?}", BitmaskVecDebug::none()); // BitmaskVecDebug[]
    println!("{:?}", BitmaskVecDebug::Flag1); // BitmaskVecDebug[Flag1]
    println!("{:?}", BitmaskVecDebug::all_flags()); // BitmaskVecDebug[Flag1, Flag2]

    let bm = BitmaskFlagsIter::Flag1;
    for flag @ &(_name, value) in BitmaskFlagsIter::flags() {
        if bm.contains(value) {
            println!("{:?}", flag) // ("Flag1", BitmaskFlagsIter { bits: 1 })
        } else {
            println!("{:?}", flag) // ("Flag2", BitmaskFlagsIter { bits: 2 })
        }
    }
}
```

### Available Config Options

- `inverted_flags` => Adds an inverted flag for every non-inverted flag to the bitmask.
- `vec_debug` => Replaces the default Debug trait implementation with a custom one that prints the bitmask as a vec of all matching values.
- `flags_iter` => Adds a `::flags()` method that returns an iterator over all flags of the bitmask represented as a tuple `(name, flag)`.

If you need / can think of any other config option, feel free to suggest them and we can discuss implementing them.

## Implemented Methods
```rust,ignore
// Returns the underlying bits of the bitmask.
const fn bits(&self) -> #type;

// Returns an iterator over all flags of the bitmask.
// Where each Item = (name, flag).
//
// This requires the `flags_iter` config option.
fn flags() -> core::iter::Iterator<Item = &'static (&'static str, Self)>;

// Returns a bitmask that contains all values.
//
// This will include bits that do not have any associated flags.
// Use `::all_flags()` if you only want to use flags.
const fn all_bits() -> Self;

// Returns `true` if the bitmask contains all values.
//
// This will check for `bits == !0`,
// use `.is_all_flags()` if you only want to check for all flags
const fn is_all_bits(&self) -> bool;

// Returns a bitmask that does not contain any values.
const fn none() -> Self;

// Returns `true` if the bitmask does not contain any values.
const fn is_none(&self) -> bool;

// Returns a bitmask that contains all flags.
const fn all_flags() -> Self;

// Returns `true` if the bitmask contains all flags.
//
// This will fail if any unused bit is set,
// consider using `.truncate()` first.
const fn is_all_flags(&self) -> bool;

// Returns a bitmask that only has bits corresponding to flags
const fn truncate(&self) -> Self;

// Returns `true` if `self` intersects with any value in `other`,
// or if `other` does not contain any values.
// This is equivalent to `(self & other) != 0 || other == 0`.
const fn intersects(&self, other: Self) -> bool;

// Returns `true` if `self` contains all values of `other`.
// This is equivalent to  `(self & other) == other`.
const fn contains(&self, other: Self) -> bool;

// Constant bitwise operations.
const fn not(self) -> Self;
const fn and(self, other: Self) -> Self;
const fn or(self, other: Self) -> Self;
const fn xor(self, other: Self) -> Self;
```

## Implemented Traits
```rust,ignore
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]

impl core::ops::Not;

impl core::ops::BitAnd;
impl core::ops::BitAndAssign;

impl core::ops::BitOr;
impl core::ops::BitOrAssign;

impl core::ops::BitXor;
impl core::ops::BitXorAssign;

impl From<#type> for #ident;
impl From<#ident> for #type;

impl PartialEq<#type>;

impl core::fmt::Binary;
impl core::fmt::LowerHex;
impl core::fmt::UpperHex;
impl core::fmt::Octal;
```
