use bitmask_enum::bitmask;

#[bitmask(u8)]
enum Bitmask {
    Flag1 = 0b00010000,
    Flag2 = 0b00000100,
    Flag3 = 0b00000001,

    // Needs const bin op methods
    Flag13 = Self::Flag1.or(Self::Flag3),
}

fn main() {
    let bm = Bitmask::Flag1 | Bitmask::Flag3;
    println!("{:#010b}", bm); // 0b00010001
    println!("{}", bm == Bitmask::Flag13); // true
}
