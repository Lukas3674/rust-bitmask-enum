#[cfg(test)]
mod tests {
    use bitmask_enum::bitmask;

    #[bitmask]
    enum Bitmask {
        Flag0,
        Flag1,
        Flag2,
        Flag3,
        Flag4,
        Flag5,
        Flag6,
        Flag7,
        Flag8,
        Flag9,
    }

    #[test]
    fn test() {
        let mut bm = Bitmask::none();
        assert_eq!(bm, 0usize);

        bm |= Bitmask::Flag5;
        assert_eq!(bm, Bitmask::Flag5);

        bm |= Bitmask::Flag1 | Bitmask::Flag9;
        assert_eq!(bm, 0b1000100010);

        bm &= !Bitmask::Flag1 & !Bitmask::Flag5;
        assert_eq!(bm, 0b1000000000);

        bm |= !Bitmask::Flag9;
        assert_eq!(bm.is_all(), true);
    }

    #[test]
    fn test_all() {
        let all = Bitmask::all();
        assert_eq!(all.is_all(), true);
        assert_eq!(all, std::usize::MAX);
    }

    #[test]
    fn test_none() {
        let none = Bitmask::none();
        assert_eq!(none.is_none(), true);
        assert_eq!(none, std::usize::MIN);
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
}
