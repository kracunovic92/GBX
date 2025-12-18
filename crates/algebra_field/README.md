# algebra_field

`algebra_field` provides concrete implementations of mathematical fields built on top of the
abstract algebraic traits from `algebra_core`.  
The main field type in this crate is `Zp<P>`, a prime field defined modulo a type-level constant
`P`.

## Features

- Prime fields `Zp<P>` with type-level modulus
- Modular addition, subtraction, multiplication, and negation
- Multiplicative inverse and safe division (`TryInverse`, `CheckedDiv`)
- Optional modulus validation (`validate-modulus` feature)
- Integrates with traits such as `Field`, `Ring`, and `Semiring` from `algebra_core`

## Example

```rust
use algebra_field::Zp;
use algebra_core::{Zero, One, TryInverse, CheckedDiv};

type F7 = Zp<7>;

let a = F7::new(5);
let b = F7::new(3);

// 5 + 3 = 1 (mod 7)
assert_eq!((a + b).value(), 1);

// inverse of 5 is 3 (mod 7)
let inv = a.try_inv().unwrap();
assert_eq!(inv.value(), 3);

// safe division
let q = a.checked_div(b).unwrap();
assert_eq!((q * b).value(), a.value());
