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
