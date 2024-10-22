use bitmask_enum::bitmask;

use std::fmt;

#[bitmask(u8)]
#[bitmask_config(flags_iter)]
enum Bitmask {
    Flag1, // defaults to 0d00000001
    Flag2, // defaults to 0d00000010
    Flag3, // defaults to 0d00000100
}

impl fmt::Display for Bitmask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(Self::flags().filter_map(|&(name, value)| self.contains(value).then(|| name)))
            .finish()
    }
}

fn main() {
    // Bitmask that contains Flag1 and Flag3
    let bm = Bitmask::Flag1 | Bitmask::Flag3;

    println!("{}", bm); // ["Flag1", "Flag3"]
}
