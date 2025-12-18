# algebra_core

Foundational algebraic traits and law-test helpers for Rust.

This crate defines **interfaces only** (traits) for basic algebraic structures:

- additive and multiplicative operations,
- semigroups, monoids, groups,
- semirings, rings, and fields,
- plus a small set of identity / scalar abstractions.

It intentionally **does not** implement these traits for foreign types (like
`Vec<T>`, `BigInt`, etc.) to respect Rust’s orphan rules and keep semantics
predictable. Concrete implementations live in sibling crates such as:

- [`algebra_field`] – finite fields like `Zp<P>`
- `algebra_poly` (planned) – multivariate polynomials over a field

> This crate is part of a larger project exploring Gröbner bases and
> computational algebra in Rust.

## Design goals

- **Small, composable traits**  
  Separate “raw operations” from algebraic structure:
    - `Additive` / `Multiplicative` (just `+` and `*`)
    - `AddSemigroup`, `AddMonoid`, `AddGroup`, `AddAbelianGroup`
    - `MulSemigroup`, `MulMonoid`, `MulAbelianMonoid`
    - `Semiring`, `Ring`, `Field`

- **Law-focused**  
  The Rust type system cannot enforce associativity or distributivity, so all
  algebraic laws are:
    - documented in trait-level docs, and
    - checked via *generic law tests* in this crate and in consumer crates.

- **No surprise blanket impls for std types**  
  We do provide `Zero` / `One` for standard integer types, but we do not
  silently treat everything as a ring/field without explicit bounds.

## Example: a tiny field-like type

```rust
use algebra_core::prelude::*;
use std::ops::{Add, Mul, Neg};

/// ℤ/7ℤ – integers modulo 7
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Mod7(u8);

impl Mod7 {
    fn new(n: u8) -> Self {
        Self(n % 7)
    }
}

impl Zero for Mod7 {
    const ZERO: Self = Mod7(0);
}

impl One for Mod7 {
    const ONE: Self = Mod7(1);
}

impl Add for Mod7 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Mod7::new(self.0 + rhs.0)
    }
}

impl Neg for Mod7 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if self.0 == 0 { self } else { Mod7(7 - self.0) }
    }
}

impl Mul for Mod7 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Mod7::new(self.0 * rhs.0)
    }
}

impl TryInverse for Mod7 {
    type Output = Self;

    fn try_inv(self) -> Option<Self::Output> {
        if self.0 == 0 {
            return None;
        }

        for x in 1..7 {
            if Mod7::new(self.0 * x).0 == 1 {
                return Some(Mod7::new(x));
            }
        }
        None
    }
}

fn is_field<F: Field>(_x: F) {}

fn main() {
    let a = Mod7::new(3);
    let b = Mod7::new(5);

    let sum = a.add(b);
    let prod = a.mul(b);

    is_field(a);
    println!("{sum:?} * {prod:?}");
}
