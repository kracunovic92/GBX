#[cfg(test)]
mod tests {
    use crate::{AddGroup, AddMonoid, Additive};

    #[test]
    fn add_i32_and_group_ops() {
        assert_eq!(3i32.add(4), 7);
        let five: i32 = 5;
        let neg_five = AddGroup::neg(five);
        assert_eq!(neg_five, -5);
    }

    #[test]
    fn add_u32_is_monoid_but_not_group() {
        assert_eq!(3u32.add(4), 7);

        fn _requires_monoid<T: AddMonoid>(_x: T) {}
        _requires_monoid(3u32);
    }
}
