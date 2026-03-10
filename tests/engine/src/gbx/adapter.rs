//! GBX adapter: runtime dispatch + ring construction.
//!
//! This file stays intentionally thin:
//! - validate `TestCase`
//! - build a `RingCtx`
//! - delegate to `gbx::run`

use crate::gbx::run::{compute_basis_in_ring, GbxRunOutput};
use crate::utils::test_file_config::TestCase;
use anyhow::{bail, Result};

use gbx_field::fp::FpDyn;
use gbx_poly::order::{Grevlex, Lex};
use gbx_poly::ring::Ring;

pub fn gbx_compute_basis(case: &TestCase) -> Result<GbxRunOutput> {
    match case.field.as_str() {
        "Fp" => gbx_compute_fp_dyn(case),
        other => bail!("unsupported GBX field '{}'", other),
    }
}

fn gbx_compute_fp_dyn(case: &TestCase) -> Result<GbxRunOutput> {
    if case.p <= 1 {
        bail!("field=Fp requires p>1, got {}", case.p);
    }
    if case.vars.is_empty() {
        bail!("vars must be non-empty");
    }

    // build field ctx
    let field = FpDyn::prime(case.p)?;

    // build ring ctx based on order
    match case.order.as_str() {
        "lex" | "lp" => {
            let ring = Ring::builder()
                .field(field)
                .order(Lex)
                .nvars(case.vars.len())
                .build()?;
            compute_basis_in_ring(&ring, case)
        }
        "dp" | "grevlex" => {
            let ring = Ring::builder()
                .field(field)
                .order(Grevlex)
                .nvars(case.vars.len())
                .build()?;
            compute_basis_in_ring(&ring, case)
        }
        other => bail!("unsupported order '{}'. Supported: lex, dp/grevlex", other),
    }
}
