#![allow(missing_docs)]
use gbx_alg::{laws, One, Ring, Zero};
use gbx_field::zp::Zp;

/// Domain-specific ring element that wraps Zp<P>.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct MyRing<const P: u32>(Zp<P>);

impl<const P: u32> MyRing<P> {
    pub fn new(x: u32) -> Self {
        Self(Zp::<P>::new(x))
    }
    pub fn value(self) -> u32 {
        self.0.value()
    }
}

impl<const P: u32> Zero for MyRing<P> {
    const ZERO: Self = Self(Zp::<P>::from_reduced_unchecked(0));
}
impl<const P: u32> One for MyRing<P> {
    const ONE: Self = Self(Zp::<P>::from_reduced_unchecked(1));
}

// Operations delegate to inner Zp<P>
impl<const P: u32> core::ops::Add for MyRing<P> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
impl<const P: u32> core::ops::Sub for MyRing<P> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}
impl<const P: u32> core::ops::Neg for MyRing<P> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}
impl<const P: u32> core::ops::Mul for MyRing<P> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

fn main() {
    // Compile-time capability check: "this type is a Ring".
    fn needs_ring<R: Ring>(_x: R) {}
    needs_ring(MyRing::<8>::one());

    // Law check on a finite sample set (all residues).
    let elems: Vec<MyRing<8>> = Zp::<8>::iter_all().map(MyRing).collect();
    laws::check_ring(&elems);

    let a = MyRing::<8>::new(7);
    let b = MyRing::<8>::new(6);
    println!("MyRing<8>: a*b = {}", (a * b).value());
}
