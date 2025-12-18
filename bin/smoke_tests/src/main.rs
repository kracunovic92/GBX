#![allow(missing_docs)]

use graph::encoding::{build_k_coloring_system, BoolPolyBuilder};
use graph::io::read_dimacs_file;

use algebra_core::Multiplicative;
use algebra_core::One;
use algebra_field::Zp;

use algebra_poly::monomial::{DynamicMonomial, Lex};
use algebra_poly::polynomial::{DynamicPolynomial, PolynomialMut};
use algebra_poly::term::{DynamicTerm, TermLike};

type F = Zp<7>;
type P = DynamicPolynomial<F, Lex>;

#[derive(Debug, Clone)]
struct DynPolyBuilder {
    num_vars: usize,
}

impl DynPolyBuilder {
    fn new(num_vars: usize) -> Self {
        Self { num_vars }
    }

    fn monomial_var(&self, index: usize) -> DynamicMonomial {
        // DynamicMonomial stores u32 exponents in your implementation.
        let mut exps = vec![0u64; self.num_vars];
        exps[index] = 1;
        DynamicMonomial::from_vec(exps)
    }

    fn one_poly(&self) -> P {
        // constant 1 = 1 * (all-zero monomial)
        P::from_terms(vec![DynamicTerm::new(
            F::one(),
            DynamicMonomial::one(self.num_vars),
        )])
    }

    fn var_poly(&self, index: usize) -> P {
        P::from_terms(vec![DynamicTerm::new(F::one(), self.monomial_var(index))])
    }
}

impl BoolPolyBuilder for DynPolyBuilder {
    type Poly = P;

    fn zero(&self) -> Self::Poly {
        P::zero()
    }

    fn one(&self) -> Self::Poly {
        self.one_poly()
    }

    fn var(&self, index: usize) -> Self::Poly {
        assert!(
            index < self.num_vars,
            "var index {index} out of range (num_vars={})",
            self.num_vars
        );
        self.var_poly(index)
    }

    fn add(&self, a: &Self::Poly, b: &Self::Poly) -> Self::Poly {
        // Trait-based add (works regardless of whether `Add` is implemented)
        P::add_poly(a, b)
    }

    fn sub(&self, a: &Self::Poly, b: &Self::Poly) -> Self::Poly {
        // Trait-based sub (already in PolynomialMut in your refactor)
        P::sub_poly(a, b)
    }

    fn mul(&self, a: &Self::Poly, b: &Self::Poly) -> Self::Poly {
        // Minimal multiplication for encoding.
        // If you already have Mul implemented, you can do `a.clone() * b.clone()`.
        // Otherwise: naive term cross-product + normalize via from_terms.
        if a.is_zero() || b.is_zero() {
            return P::zero();
        }

        let mut out = Vec::with_capacity(
            a.terms()
                .len()
                * b.terms()
                    .len(),
        );
        for ta in a.terms() {
            for tb in b.terms() {
                let coeff = ta
                    .coeff()
                    .clone()
                    .mul(
                        tb.coeff()
                            .clone(),
                    );
                let mono = ta
                    .mono()
                    .mul(tb.mono()); // uses your DynamicMonomial::mul
                out.push(DynamicTerm::new(coeff, mono));
            }
        }
        P::from_terms(out)
    }

    fn pow(&self, a: &Self::Poly, exp: u32) -> Self::Poly {
        match exp {
            0 => self.one_poly(),
            1 => a.clone(),
            _ => {
                let mut acc = self.one_poly();
                for _ in 0..exp {
                    acc = self.mul(&acc, a);
                }
                acc
            }
        }
    }
}

fn usage() -> ! {
    eprintln!("Usage:");
    eprintln!("  cargo run -p smoke_tests -- <path/to/file.col> <k> [--limit N] [--no-print]");
    eprintln!();
    eprintln!("Example:");
    eprintln!("  cargo run -p smoke_tests -- instances/huck.col 3 --limit 30");
    std::process::exit(2);
}

fn main() {
    let mut args = std::env::args().skip(1);

    let path = args
        .next()
        .unwrap_or_else(|| usage());
    let k: usize = args
        .next()
        .unwrap_or_else(|| usage())
        .parse()
        .unwrap_or_else(|_| usage());

    let mut limit: usize = 50;
    let mut print = true;

    while let Some(a) = args.next() {
        match a.as_str() {
            "--limit" => {
                let n = args
                    .next()
                    .unwrap_or_else(|| usage());
                limit = n
                    .parse()
                    .unwrap_or_else(|_| usage());
            }
            "--no-print" => print = false,
            _ => usage(),
        }
    }

    let g = read_dimacs_file(&path).unwrap_or_else(|e| {
        eprintln!("DIMACS parse failed for '{path}': {e}");
        std::process::exit(1);
    });

    let num_vars = g.n * k;

    println!("Loaded: {path}");
    println!("Graph: n={}, m={}", g.n, g.m);
    println!("k={k}, num_vars={num_vars}");
    println!();

    let builder = DynPolyBuilder::new(num_vars);
    let enc = build_k_coloring_system(&g, k, &builder);

    let expected = g.n * k + g.n + g.m * k;
    println!("Encoding summary:");
    println!(
        "  polynomials    = {}",
        enc.polynomials
            .len()
    );
    println!("  expected_polys = {}", expected);
    println!("  num_vars       = {}", enc.num_vars());
    println!();

    if !print {
        return;
    }

    let to_print = enc
        .polynomials
        .len()
        .min(limit);
    println!("First {to_print} polynomials:");
    println!("------------------------------------");
    for (i, p) in enc
        .polynomials
        .iter()
        .take(to_print)
        .enumerate()
    {
        println!("{:>6}: {}", i, p);
    }
}
