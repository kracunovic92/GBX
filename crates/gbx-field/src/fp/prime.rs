const BASES: [u32; 5] = [2, 3, 5, 7, 11];

#[inline]
pub fn is_prime_u32(candidate: u32) -> bool {
    if candidate < 2 {
        return false;
    }

    for &small_prime in &[2u32, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37] {
        if candidate == small_prime {
            return true;
        }

        if candidate % small_prime == 0 {
            return false;
        }
    }

    let candidate64 = u64::from(candidate);

    let mut odd_part = u64::from(candidate - 1);
    let mut two_adic_power = 0u32;

    while (odd_part & 1) == 0 {
        odd_part >>= 1;
        two_adic_power += 1;
    }

    'outer: for &base32 in &BASES {
        if base32 >= candidate {
            continue;
        }

        let base = u64::from(base32);
        let mut witness = pow_mod(base, odd_part, candidate64);

        if witness == 1 || witness == candidate64 - 1 {
            continue;
        }

        for _ in 1..two_adic_power {
            witness = mul_mod(witness, witness, candidate64);

            if witness == candidate64 - 1 {
                continue 'outer;
            }
        }

        return false;
    }

    true
}

const fn mul_mod(lhs: u64, rhs: u64, modulus: u64) -> u64 {
    (lhs * rhs) % modulus
}

const fn pow_mod(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut acc = 1u64 % modulus;

    while exponent > 0 {
        if (exponent & 1) == 1 {
            acc = mul_mod(acc, base, modulus);
        }

        base = mul_mod(base, base, modulus);
        exponent >>= 1;
    }

    acc
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

        assert!(!is_prime_u32(0));
        assert!(!is_prime_u32(1));
        assert!(!is_prime_u32(9));
        assert!(!is_prime_u32(21));
    }
}
