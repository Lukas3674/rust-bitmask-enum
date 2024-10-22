#![no_std]

#[cfg(test)]
mod tests {
    use bitmask_enum::bitmask;

    #[bitmask]
    #[bitmask_config(inverted_flags, vec_debug, flags_iter)]
    enum NoStdBitmask {
        Flag1,
        Flag2,
    }

    #[bitmask(u8)]
    #[bitmask_config(inverted_flags, vec_debug, flags_iter)]
    enum NoStdBitmaskU8 {
        Flag1,
        Flag2,
    }

    #[test]
    fn test() {
        let mut bm = NoStdBitmask::none();
        assert_eq!(bm, 0);

        bm |= NoStdBitmask::Flag1;
        assert_eq!(bm, NoStdBitmask::Flag1);

        bm |= NoStdBitmask::InvertedFlag1;
        assert_eq!(bm.is_all_bits(), true);

        let mut bm = NoStdBitmaskU8::none();
        assert_eq!(bm, 0);

        bm |= NoStdBitmaskU8::Flag1;
        assert_eq!(bm, NoStdBitmaskU8::Flag1);

        bm |= NoStdBitmaskU8::InvertedFlag1;
        assert_eq!(bm.is_all_bits(), true);
    }
}