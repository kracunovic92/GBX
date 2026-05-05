use anyhow::{Context, Result};

use gbx_field::fp::{Fp, FpElem};
use gbx_grobner::{f4, F4Options};
use gbx_poly::monomial::Monomial;
use gbx_poly::order::Lex;
use gbx_poly::poly;
use gbx_poly::polynomial::Polynomial;
use gbx_poly::ring::{Ring, RingCtx};
use gbx_poly::term::Term;

#[cfg(feature = "profiling-alloc")]
use stats_alloc::{StatsAlloc, INSTRUMENTED_SYSTEM};

#[cfg(feature = "profiling-alloc")]
#[global_allocator]
static GLOBAL: &StatsAlloc<std::alloc::System> = &INSTRUMENTED_SYSTEM;

type P = Polynomial<FpElem>;

fn main() -> Result<()> {
    init_tracing();

    let field = Fp::prime(32003).context("invalid modulus p")?;

    let ring = Ring::builder().field(field).order(Lex).nvars(6).build()?;

    let generators = cyclic6_generators(&ring)?;

    println!("case=cyclic6");
    println!("field=Fp(32003)");
    println!("order=lp");
    println!("nvars=6");
    println!("generators={}", generators.len());

    let opts = F4Options::default();

    let started = std::time::Instant::now();

    let gb = f4(&ring, generators.iter().cloned(), opts)?;

    let elapsed = started.elapsed();

    println!("basis_size={}", gb.as_slice().len());
    println!("elapsed_ms={}", elapsed.as_millis());

    Ok(())
}

#[cfg(feature = "instrumentation")]
fn init_tracing() {
    use tracing_subscriber::fmt::format::FmtSpan;
    use tracing_subscriber::{fmt, EnvFilter};

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,gbx_grobner=debug,baseline_experiments=debug"));

    fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_level(true)
        .with_span_events(FmtSpan::CLOSE)
        .compact()
        .init();
}

#[cfg(not(feature = "instrumentation"))]
fn init_tracing() {}

fn cyclic6_generators(ring: &RingCtx<Fp, Lex>) -> Result<Vec<P>> {
    let g1: P = poly![
        ring;
        (1, [1, 0, 0, 0, 0, 0]),
        (1, [0, 1, 0, 0, 0, 0]),
        (1, [0, 0, 1, 0, 0, 0]),
        (1, [0, 0, 0, 1, 0, 0]),
        (1, [0, 0, 0, 0, 1, 0]),
        (1, [0, 0, 0, 0, 0, 1])
    ]?;

    let g2 = cyclic_sum(ring, 2)?;
    let g3 = cyclic_sum(ring, 3)?;
    let g4 = cyclic_sum(ring, 4)?;
    let g5 = cyclic_sum(ring, 5)?;

    let g6: P = poly![
        ring;
        (1_u32, [1, 1, 1, 1, 1, 1]),
        (32002_u32, [0, 0, 0, 0, 0, 0])
    ]?;

    Ok(vec![g1, g2, g3, g4, g5, g6])
}

fn cyclic_sum(ring: &RingCtx<Fp, Lex>, width: usize) -> Result<P> {
    debug_assert!((1..=5).contains(&width));

    let terms = (0..6)
        .map(|start| {
            let mut exps = [0_u32; 6];

            for offset in 0..width {
                let idx = (start + offset) % 6;
                exps[idx] = 1;
            }

            (1_u32, exps)
        })
        .collect::<Vec<_>>();

    poly_from_terms(ring, &terms)
}

fn poly_from_terms(ring: &RingCtx<Fp, Lex>, terms: &[(u32, [u32; 6])]) -> Result<P> {
    let terms = terms
        .iter()
        .map(|(coeff, exps)| Term::new(ring.field.new(*coeff), Monomial::from_slice(exps)))
        .collect::<Vec<_>>();

    P::from_terms_in(ring, terms).map_err(Into::into)
}
