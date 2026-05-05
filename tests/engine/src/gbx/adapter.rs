//! GBX adapter: runtime dispatch + ring construction.

use crate::gbx::config::GbxConfig;
use crate::gbx::run::{compute_basis_in_ring, GbxRunOutput};
use crate::utils::test_file_config::TestCase;
use anyhow::{bail, Result};

use gbx_field::fp::Fp;
use gbx_poly::order::{Grevlex, Lex};
use gbx_poly::ring::Ring;

#[tracing::instrument(
    skip_all,
    fields(
        case = %case.name,
        field = %case.field,
        order = %case.order,
        reducer = %cfg.reducer.as_str()
    )
)]
pub fn gbx_compute_basis(case: &TestCase, cfg: &GbxConfig) -> Result<GbxRunOutput> {
    match case.field.as_str() {
        "Fp" => gbx_compute_fp_dyn(case, cfg),
        other => bail!("unsupported GBX field '{other}'"),
    }
}

fn gbx_compute_fp_dyn(case: &TestCase, cfg: &GbxConfig) -> Result<GbxRunOutput> {
    if case.p <= 1 {
        bail!("field=Fp requires p>1, got {}", case.p);
    }

    if case.vars.is_empty() {
        bail!("vars must be non-empty");
    }

    let field = Fp::prime(case.p)?;

    match case.order.as_str() {
        "lex" | "lp" => {
            let ring = Ring::builder()
                .field(field)
                .order(Lex)
                .nvars(case.vars.len())
                .build()?;

            compute_basis_in_ring(&ring, case, cfg)
        }

        "dp" | "grevlex" => {
            let ring = Ring::builder()
                .field(field)
                .order(Grevlex)
                .nvars(case.vars.len())
                .build()?;

            compute_basis_in_ring(&ring, case, cfg)
        }

        other => bail!("unsupported order '{other}'. Supported: lex, lp, dp, grevlex"),
    }
}
