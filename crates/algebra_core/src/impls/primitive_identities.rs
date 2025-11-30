use crate::traits::identity::{One, Scalar, Zero};

macro_rules! impl_zero_one_for_ints {
    ($($t:ty),* $(,)?) => {
        $(
            impl Zero for $t {
                const ZERO: Self = 0;
            }

            impl One for $t {
                const ONE: Self = 1;
            }
        )*
    };
}
impl_zero_one_for_ints!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize,
);

impl<T> Scalar for T where T: Zero + One + Copy + PartialEq {}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_identities<T>(z: T, o: T)
    where
        T: Zero + One + Copy + PartialEq + core::fmt::Debug,
    {
        assert_eq!(T::zero(), z);
        assert_eq!(T::one(), o);
        assert!(z.is_zero());
        assert!(!o.is_zero());
        assert!(o.is_one());
        assert!(!z.is_one());
    }

    #[test]
    fn integers_have_zero_and_one() {
        check_identities::<u32>(0, 1);
        check_identities::<i64>(0, 1);
        check_identities::<usize>(0, 1);
    }
}
