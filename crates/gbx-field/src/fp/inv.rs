use gbx_alg::TryInverse;

use super::Fp;

impl<const P: u32> TryInverse for Fp<P> {
    type Output = Self;

    #[inline]
    fn try_inv(self) -> Option<Self::Output> {
        if self.value() == 0 {
            return None;
        }

        // Extended Euclid on (a, P).
        let mut a = self.value() as i64;
        let mut b = P as i64;
        let mut x0: i64 = 1;
        let mut x1: i64 = 0;

        while b != 0 {
            let q = a / b;
            (a, b) = (b, a - q * b);
            (x0, x1) = (x1, x0 - q * x1);
        }

        // In a true prime field gcd must be 1 for all nonzero a.
        if a != 1 {
            return None;
        }

        let inv = x0.rem_euclid(P as i64) as u32;
        Some(Self::from_reduced_unchecked(inv))
    }
}
