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

    // Does bm contain / intersect CONST_BM
    println!("{}", bm.contains(CONST_BM)); // true

    // Does bm contain / intersect all of CONST_BM
    println!("{}", bm.contains_all(CONST_BM)); // false
}
```

## Custom Values

You can assign every flag a custom value, but every flag requires a unique value that only contains one `1` bit.

```rust
use bitmask_enum::bitmask;

#[bitmask(u8)]
enum Bitmask {
    Flag1 = 0b00010000,
    Flag2 = 0b00000100,
    Flag3 = 0b00000001,
}

fn main() {
    let bm = Bitmask::Flag1 | Bitmask::Flag3;
    println!("{:#010b}", bm); // 0b00010001
}
```
