# Bitmask-Enum

[![API](https://docs.rs/bitmask-enum/badge.svg)](https://docs.rs/bitmask-enum)
[![Crate](https://img.shields.io/crates/v/bitmask-enum.svg)](https://crates.io/crates/bitmask-enum)

A bitmask enum attribute macro.

A bitmask can have unsigned integer types, the default type is `usize`.

Don't know how to document in `proc-macro` crates so if you want see a better documentation run `cargo doc --open` and select your `Bitmask` enum.

```rust
#[bitmask] // usize
enum Bitmask { /* ... */ }

#[bitmask(u8)] // u8
enum Bitmask { /* ... */ }
```

## Example

```rust
use bitmask_enum::bitmask;

#[bitmask(u8)]
enum Bitmask {
    Flag1, // defaults to 0d00000001
    Flag2, // defaults to 0d00000010
    Flag3, // defaults to 0d00000100
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

You can assign every flag a custom value.

Because behind the scences `enum Bitmask` gets converted to a `struct Bitmask(u8);` you need to wrap `u8` expressions into a `Self(_)`.

```rust
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

fn main() {
    let bm = Bitmask::Flag1 | Bitmask::Flag3;

    println!("{:#010b}", bm); // 0b00010001
    println!("{}", bm == Bitmask::Flag13_1); // true
    println!("{}", bm == Bitmask::Flag13_2); // true

    println!("{:#010b}", Bitmask::Flag4); // 0b00010101
}
```
