use bitmask_enum::bitmask;

#[bitmask(u8)]
enum Bitmask {
    Flag1 = 0b00010000,
    Flag2 = 0b00000100,
    Flag3 = 0b00000001,

    // Needs const methods
    // Self::Flag1 | Self::Flag3    not possible
    // 0b00010000 | 0b00000001      possible
    Flag13_1 = 0b00010000 | 0b00000001,
    Flag13_2 = Self::Flag1.or(Self::Flag3),
}

fn main() {
    let bm = Bitmask::Flag1 | Bitmask::Flag3;
    println!("{:#010b}", bm); // 0b00010001
    println!("{}", bm == Bitmask::Flag13_1); // true
    println!("{}", bm == Bitmask::Flag13_2); // true
}
