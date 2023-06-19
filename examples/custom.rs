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
