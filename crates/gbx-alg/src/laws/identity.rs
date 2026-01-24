use crate::{One, Zero};

pub fn check_zero_laws<T>(elems: &[T])
where
    T: Copy + core::fmt::Debug + PartialEq + Zero + core::ops::Add<Output = T>,
{
    let z = T::zero();
    for &a in elems {
        assert_eq!(z + a, a, "0 + a == a failed for a={a:?}");
        assert_eq!(a + z, a, "a + 0 == a failed for a={a:?}");
    }
}

pub fn check_one_laws<T>(elems: &[T])
where
    T: Copy + core::fmt::Debug + PartialEq + One + core::ops::Mul<Output = T>,
{
    let o = T::one();
    for &a in elems {
        assert_eq!(o * a, a, "1 * a == a failed for a={a:?}");
        assert_eq!(a * o, a, "a * 1 == a failed for a={a:?}");
    }
}
