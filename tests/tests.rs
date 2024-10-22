#[cfg(test)]
mod tests {
    use bitmask_enum::bitmask;

    #[bitmask]
    enum Bitmask {
        Flag1,
        Flag2,
        Flag3,
        Flag4,
        Flag5,
        Flag6,
        Flag7,
        Flag8,
    }

    #[test]
    fn test() {
        let mut bm = Bitmask::none();
        assert_eq!(bm, 0);

        bm |= Bitmask::Flag5;
        assert_eq!(bm, Bitmask::Flag5);

        bm |= Bitmask::Flag1 | Bitmask::Flag8;
        assert_eq!(bm, 0b10010001);

        bm &= !Bitmask::Flag1 & !Bitmask::Flag5;
        assert_eq!(bm, 0b10000000);

        bm |= !Bitmask::Flag8;
        assert_eq!(bm.is_all_bits(), true);
    }

    #[test]
    fn test_bits() {
        let all = Bitmask::all_bits();
        assert_eq!(all.bits(), std::usize::MAX);
    }

    #[test]
    fn test_all_bits() {
        let all = Bitmask::all_bits();
        assert_eq!(all.is_all_bits(), true);
        assert_eq!(all, std::usize::MAX);
    }

    #[test]
    fn test_none() {
        let none = Bitmask::none();
        assert_eq!(none.is_none(), true);
        assert_eq!(none, std::usize::MIN);
    }

    #[test]

    fn test_all_flags() {
        let all_flags = Bitmask::all_flags();
        assert_eq!(all_flags.is_all_flags(), true);
        assert_eq!(
            all_flags,
            Bitmask::Flag1
                | Bitmask::Flag2
                | Bitmask::Flag3
                | Bitmask::Flag4
                | Bitmask::Flag5
                | Bitmask::Flag6
                | Bitmask::Flag7
                | Bitmask::Flag8
        );
    }

    #[test]
    fn test_truncate() {
        let all = Bitmask::all_bits();
        assert_eq!(all.is_all_bits(), true);
        assert_eq!(all.is_all_flags(), false);
        assert_eq!(all.truncate().is_all_flags(), true);
    }

    #[test]
    fn test_intersects() {
        let bm = Bitmask::Flag4;
        assert_eq!(bm.intersects(Bitmask::Flag4), true);
        assert_eq!(bm.intersects(Bitmask::Flag4 | Bitmask::Flag1), true);
        assert_eq!(bm.intersects(Bitmask::Flag1), false);
    }

    #[test]
    fn test_contains() {
        let bm = Bitmask::Flag4 | Bitmask::Flag6;
        assert_eq!(bm.contains(Bitmask::Flag4), true);
        assert_eq!(bm.contains(Bitmask::Flag4 | Bitmask::Flag6), true);
        assert_eq!(bm.contains(Bitmask::Flag1), false);
        assert_eq!(bm.contains(Bitmask::Flag4 | Bitmask::Flag1), false);
    }

    #[test]
    fn test_from() {
        let mask: usize = 0b100010;
        let bm = Bitmask::from(mask);

        assert_eq!(bm, Bitmask::Flag2 | Bitmask::Flag6);

        let value: usize = bm.into();
        assert_eq!(value, mask);
    }

    #[test]
    fn test_types() {
        #[bitmask(usize)]
        enum BitmaskUsize {
            Flag1,
            Flag2,
        }
        assert_eq!(BitmaskUsize::Flag1, 0b01);
        assert_eq!(BitmaskUsize::Flag2, 0b10);

        #[bitmask(u8)]
        enum BitmaskU8 {
            Flag1,
            Flag2,
        }
        assert_eq!(BitmaskU8::Flag1, 0b01);
        assert_eq!(BitmaskU8::Flag2, 0b10);

        #[bitmask(u16)]
        enum BitmaskU16 {
            Flag1,
            Flag2,
        }
        assert_eq!(BitmaskU16::Flag1, 0b01);
        assert_eq!(BitmaskU16::Flag2, 0b10);

        #[bitmask(u32)]
        enum BitmaskU32 {
            Flag1,
            Flag2,
        }
        assert_eq!(BitmaskU32::Flag1, 0b01);
        assert_eq!(BitmaskU32::Flag2, 0b10);

        #[bitmask(u64)]
        enum BitmaskU64 {
            Flag1,
            Flag2,
        }
        assert_eq!(BitmaskU64::Flag1, 0b01);
        assert_eq!(BitmaskU64::Flag2, 0b10);

        #[bitmask(u128)]
        enum BitmaskU128 {
            Flag1,
            Flag2,
        }
        assert_eq!(BitmaskU128::Flag1, 0b01);
        assert_eq!(BitmaskU128::Flag2, 0b10);

        #[bitmask(isize)]
        enum BitmaskIsize {
            Flag1,
            Flag2,
        }
        assert_eq!(BitmaskIsize::Flag1, 0b01);
        assert_eq!(BitmaskIsize::Flag2, 0b10);

        #[bitmask(i8)]
        enum BitmaskI8 {
            Flag1,
            Flag2,
        }
        assert_eq!(BitmaskI8::Flag1, 0b01);
        assert_eq!(BitmaskI8::Flag2, 0b10);

        #[bitmask(i16)]
        enum BitmaskI16 {
            Flag1,
            Flag2,
        }
        assert_eq!(BitmaskI16::Flag1, 0b01);
        assert_eq!(BitmaskI16::Flag2, 0b10);

        #[bitmask(i32)]
        enum BitmaskI32 {
            Flag1,
            Flag2,
        }
        assert_eq!(BitmaskI32::Flag1, 0b01);
        assert_eq!(BitmaskI32::Flag2, 0b10);

        #[bitmask(i64)]
        enum BitmaskI64 {
            Flag1,
            Flag2,
        }
        assert_eq!(BitmaskI64::Flag1, 0b01);
        assert_eq!(BitmaskI64::Flag2, 0b10);

        #[bitmask(i128)]
        enum BitmaskI128 {
            Flag1,
            Flag2,
        }
        assert_eq!(BitmaskI128::Flag1, 0b01);
        assert_eq!(BitmaskI128::Flag2, 0b10);
    }

    #[test]
    fn test_custom() {
        #[bitmask]
        enum BitmaskCustom {
            Flag1,
            Flag2,
            Flag12 = Self::Flag1.or(Self::Flag2).bits,
            Flag3,
            Flag123 = Self::Flag12.or(Self::Flag3).bits,
            Flag4,
        }
        assert_eq!(BitmaskCustom::Flag1, 0b1);
        assert_eq!(BitmaskCustom::Flag2, 0b10);
        assert_eq!(BitmaskCustom::Flag12, 0b11);
        assert_eq!(BitmaskCustom::Flag3, 0b100);
        assert_eq!(BitmaskCustom::Flag123, 0b111);
        assert_eq!(BitmaskCustom::Flag4, 0b1000);

        #[bitmask(u8)]
        enum BitmaskCustomTyped {
            Flag1,
            Flag2,
            Flag12 = Self::Flag1.or(Self::Flag2).bits,
            Flag3,
            Flag123 = Self::Flag12.or(Self::Flag3).bits,
            Flag4,
        }
        assert_eq!(BitmaskCustomTyped::Flag1, 0b1);
        assert_eq!(BitmaskCustomTyped::Flag2, 0b10);
        assert_eq!(BitmaskCustomTyped::Flag12, 0b11);
        assert_eq!(BitmaskCustomTyped::Flag3, 0b100);
        assert_eq!(BitmaskCustomTyped::Flag123, 0b111);
        assert_eq!(BitmaskCustomTyped::Flag4, 0b1000);
    }

    #[test]
    fn test_inverted() {
        #[bitmask]
        #[bitmask_config(inverted_flags)]
        enum BitmaskInverted {
            Flag1,
            Flag2,
            Flag3,
            Flag4,
        }
        assert_eq!(
            BitmaskInverted::InvertedFlag1,
            BitmaskInverted::all_bits().xor(BitmaskInverted::Flag1)
        );
        assert_eq!(
            BitmaskInverted::InvertedFlag2,
            BitmaskInverted::all_bits().xor(BitmaskInverted::Flag2)
        );
        assert_eq!(
            BitmaskInverted::InvertedFlag3,
            BitmaskInverted::all_bits().xor(BitmaskInverted::Flag3)
        );
        assert_eq!(
            BitmaskInverted::InvertedFlag4,
            BitmaskInverted::all_bits().xor(BitmaskInverted::Flag4)
        );
    }

    #[test]
    fn test_type_with_inverted() {
        #[bitmask(usize)]
        #[bitmask_config(inverted_flags)]
        enum BitmaskUsize {
            Flag1,
            Flag2,
        }
        assert_eq!(BitmaskUsize::InvertedFlag1, 0b01 ^ !0);
        assert_eq!(BitmaskUsize::InvertedFlag2, 0b10 ^ !0);

        #[bitmask(u8)]
        #[bitmask_config(inverted_flags)]
        enum BitmaskU8 {
            Flag1,
            Flag2,
        }
        assert_eq!(BitmaskU8::InvertedFlag1, 0b11111110);
        assert_eq!(BitmaskU8::InvertedFlag2, 0b11111101);

        #[bitmask(i16)]
        #[bitmask_config(inverted_flags)]
        enum BitmaskI16 {
            Flag1,
            Flag2,
        }
        assert_eq!(BitmaskI16::InvertedFlag1, 0b1111111111111110u16 as i16);
        assert_eq!(BitmaskI16::InvertedFlag2, 0b1111111111111101u16 as i16);
    }

    #[test]
    fn test_custom_inverted() {
        #[bitmask]
        #[bitmask_config(inverted_flags)]
        enum BitmaskCustom {
            Flag1,
            Flag2,
            Flag12 = Self::Flag1.or(Self::Flag2).bits,
            Flag3,
            Flag123 = Self::Flag12.or(Self::Flag3).bits,
            Flag4,
        }
        assert_eq!(BitmaskCustom::Flag1, 0b1);
        assert_eq!(BitmaskCustom::Flag2, 0b10);
        assert_eq!(BitmaskCustom::Flag12, 0b11);
        assert_eq!(BitmaskCustom::Flag3, 0b100);
        assert_eq!(BitmaskCustom::Flag123, 0b111);
        assert_eq!(BitmaskCustom::Flag4, 0b1000);
        assert_eq!(BitmaskCustom::InvertedFlag1, !0b1);
        assert_eq!(BitmaskCustom::InvertedFlag2, !0b10);
        assert_eq!(BitmaskCustom::InvertedFlag12, !0b11);
        assert_eq!(BitmaskCustom::InvertedFlag3, !0b100);
        assert_eq!(BitmaskCustom::InvertedFlag123, !0b111);
        assert_eq!(BitmaskCustom::InvertedFlag4, !0b1000);

        #[bitmask(u8)]
        #[bitmask_config(inverted_flags)]
        enum BitmaskCustomTyped {
            Flag1,
            Flag2,
            Flag12 = Self::Flag1.or(Self::Flag2).bits,
            Flag3,
            Flag123 = Self::Flag12.or(Self::Flag3).bits,
            Flag4,
        }
        assert_eq!(BitmaskCustomTyped::Flag1, 0b1);
        assert_eq!(BitmaskCustomTyped::Flag2, 0b10);
        assert_eq!(BitmaskCustomTyped::Flag12, 0b11);
        assert_eq!(BitmaskCustomTyped::Flag3, 0b100);
        assert_eq!(BitmaskCustomTyped::Flag123, 0b111);
        assert_eq!(BitmaskCustomTyped::Flag4, 0b1000);
        assert_eq!(BitmaskCustomTyped::InvertedFlag1, !0b1);
        assert_eq!(BitmaskCustomTyped::InvertedFlag2, !0b10);
        assert_eq!(BitmaskCustomTyped::InvertedFlag12, !0b11);
        assert_eq!(BitmaskCustomTyped::InvertedFlag3, !0b100);
        assert_eq!(BitmaskCustomTyped::InvertedFlag123, !0b111);
        assert_eq!(BitmaskCustomTyped::InvertedFlag4, !0b1000);
    }

    #[test]
    fn test_vec_debug() {
        #[bitmask]
        #[bitmask_config(vec_debug)]
        pub enum BitmaskVecDebug {
            Flag1,
            Flag2,
            Flag12 = Self::Flag1.or(Self::Flag2).bits,
            Flag3,
        }

        assert_eq!(
            format!("{:?}", BitmaskVecDebug::none()),
            "BitmaskVecDebug[]"
        );
        assert_eq!(
            format!("{:?}", BitmaskVecDebug::Flag1),
            "BitmaskVecDebug[Flag1]"
        );
        assert_eq!(
            format!("{:?}", BitmaskVecDebug::Flag2),
            "BitmaskVecDebug[Flag2]"
        );
        assert_eq!(
            format!("{:?}", BitmaskVecDebug::Flag12),
            "BitmaskVecDebug[Flag1, Flag2, Flag12]"
        );
        assert_eq!(
            format!("{:?}", BitmaskVecDebug::Flag3),
            "BitmaskVecDebug[Flag3]"
        );
        assert_eq!(
            format!("{:?}", BitmaskVecDebug::Flag2.or(BitmaskVecDebug::Flag3)),
            "BitmaskVecDebug[Flag2, Flag3]"
        );
        assert_eq!(
            format!("{:?}", BitmaskVecDebug::all_flags()),
            "BitmaskVecDebug[Flag1, Flag2, Flag12, Flag3]"
        );

        // default formatting
        assert_eq!(format!("{:?}", Bitmask::none()), "Bitmask { bits: 0 }");
        assert_eq!(format!("{:?}", Bitmask::Flag1), "Bitmask { bits: 1 }");
        assert_eq!(format!("{:?}", Bitmask::Flag2), "Bitmask { bits: 2 }");
        assert_eq!(
            format!("{:?}", Bitmask::Flag1.or(Bitmask::Flag2)),
            "Bitmask { bits: 3 }"
        );
        assert_eq!(format!("{:?}", Bitmask::Flag3), "Bitmask { bits: 4 }");
    }

    #[test]
    fn test_vec_debug_inverted() {
        #[bitmask(u8)]
        #[bitmask_config(vec_debug, inverted_flags)]
        pub enum BitmaskVecDebug {
            Flag1,
            Flag2,
        }

        assert_eq!(
            format!("{:?}", BitmaskVecDebug::none()),
            "BitmaskVecDebug[]"
        );
        assert_eq!(
            format!("{:?}", BitmaskVecDebug::Flag1),
            "BitmaskVecDebug[Flag1]"
        );
        assert_eq!(
            format!("{:?}", BitmaskVecDebug::Flag2),
            "BitmaskVecDebug[Flag2]"
        );
        assert_eq!(
            format!("{:?}", BitmaskVecDebug::InvertedFlag1),
            "BitmaskVecDebug[InvertedFlag1, Flag2]"
        );
        assert_eq!(
            format!("{:?}", BitmaskVecDebug::InvertedFlag2),
            "BitmaskVecDebug[Flag1, InvertedFlag2]"
        );
        assert_eq!(
            format!("{:?}", BitmaskVecDebug::all_flags()),
            "BitmaskVecDebug[Flag1, InvertedFlag1, Flag2, InvertedFlag2]"
        );
    }

    #[test]
    fn test_import_debug() {
        // check that having a `Debug` import doesn't lead to a conflict
        #[allow(unused)]
        use std::fmt::Debug;

        // this should just compile
        #[bitmask]
        pub enum BitmaskImportDebug {
            Flag1,
            Flag2,
        }
    }

    #[test]
    fn test_fields_iter() {
        #[bitmask(u8)]
        #[bitmask_config(flags_iter)]
        pub enum BitmaskFlagsIter {
            Flag1,
            Flag2,
            Flag3,
            FlagCustom = 123,
        }

        assert_eq!(
            BitmaskFlagsIter::flags().copied().collect::<Vec<_>>(),
            vec![
                ("Flag1", BitmaskFlagsIter::Flag1),
                ("Flag2", BitmaskFlagsIter::Flag2),
                ("Flag3", BitmaskFlagsIter::Flag3),
                ("FlagCustom", BitmaskFlagsIter::FlagCustom),
            ],
        );
    }

    #[test]
    fn test_fields_iter_inverted() {
        #[bitmask(u8)]
        #[bitmask_config(flags_iter, inverted_flags)]
        pub enum BitmaskFlagsIter {
            Flag1,
            FlagCustom = 123,
        }

        assert_eq!(
            BitmaskFlagsIter::flags().copied().collect::<Vec<_>>(),
            vec![
                ("Flag1", BitmaskFlagsIter::Flag1),
                ("InvertedFlag1", BitmaskFlagsIter::InvertedFlag1),
                ("FlagCustom", BitmaskFlagsIter::FlagCustom),
                ("InvertedFlagCustom", BitmaskFlagsIter::InvertedFlagCustom),
            ],
        );
    }
}
