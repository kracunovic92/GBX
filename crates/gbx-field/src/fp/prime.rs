#[cfg(feature = "prime-check")]
use std::sync::OnceLock;

#[cfg(feature = "prime-check")]
#[inline]
pub(crate) fn validate_prime_once<const P: u32>() {
    static OK: OnceLock<bool> = OnceLock::new();
    let ok = OK.get_or_init(|| P >= 2 && is_prime_u32(P));
    assert!(*ok, "Fp<P>: modulus P={P} must be prime and >= 2");
}

#[inline]
pub(crate) fn is_prime_u32(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    // quick small primes
    for &p in &[2u32, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if n == p {
            return true;
        }
        if n % p == 0 {
            return false;
        }
    }

    // Deterministic Miller–Rabin bases for 32-bit ints
    fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
        (a * b) % m
    }
    fn pow_mod(mut a: u64, mut e: u64, m: u64) -> u64 {
        let mut acc = 1u64 % m;
        while e > 0 {
            if (e & 1) == 1 {
                acc = mul_mod(acc, a, m);
            }
            a = mul_mod(a, a, m);
            e >>= 1;
        }
        acc
    }

    let n64 = n as u64;
    let mut d = (n - 1) as u64;
    let mut s = 0u32;
    while (d & 1) == 0 {
        d >>= 1;
        s += 1;
    }

    const BASES: [u32; 5] = [2, 3, 5, 7, 11];
    'outer: for &a32 in &BASES {
        if a32 >= n {
            continue;
        }
        let a = a32 as u64;
        let mut x = pow_mod(a, d, n64);
        if x == 1 || x == n64 - 1 {
            continue;
        }
        for _ in 1..s {
            x = mul_mod(x, x, n64);
            if x == n64 - 1 {
                continue 'outer;
            }
        }
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::is_prime_u32;

    #[test]
    fn prime_smoke() {
        assert!(is_prime_u32(2));
        assert!(is_prime_u32(3));
        assert!(is_prime_u32(7));
        assert!(is_prime_u32(998_244_353));
        assert!(!is_prime_u32(1));
        assert!(!is_prime_u32(9));
        assert!(!is_prime_u32(21));
    }
}
