//! Core algebraic trait hierarchy.

pub mod additive;
pub mod field;
pub mod identity;
pub mod multiplicative;
pub mod ring;
pub mod semiring;

pub use additive::{AddAbelianGroup, AddGroup, AddMonoid, AddSemigroup, Additive, AdditiveAssign};
pub use field::{CheckedDiv, DivByZero, Field, TryInverse};
pub use identity::{One, Scalar, Zero};
pub use multiplicative::{MulAbelianMonoid, MulMonoid, MulSemigroup, Multiplicative, MultiplicativeAssign};
pub use ring::Ring;
pub use semiring::Semiring;
