use crate::algos::f4::error::Result;
use crate::algos::f4::options::F4Options;
use crate::algos::f4::pairs::criterion::ProductCriterion;
use crate::algos::f4::pairs::update::update_with_polynomial;
use crate::algos::f4::state::F4State;

use gbx_poly::order::MonomialOrder;
use gbx_poly::polynomial::{PolynomialMut, PolynomialOps, PolynomialView};
use gbx_poly::ring::{FieldCtx, RingCtx};

#[cfg_attr(feature = "instrumentation", tracing::instrument(level = "debug", name = "f4.initialize_state", skip(ctx, fs, opts, criterion),))]
pub fn initialize_state<P, F, O>(ctx: &RingCtx<F, O>, fs: impl IntoIterator<Item = P>, opts: &F4Options, criterion: &ProductCriterion) -> Result<F4State<P>>
where
    F: FieldCtx<Elem = P::Coeff>,
    O: MonomialOrder + Clone,
    P: PolynomialMut + PolynomialOps + PolynomialView + Clone,
    P::Coeff: Copy + Eq + Default,
{
    #[cfg(feature = "profiling-alloc")]
    let alloc_region = stats_alloc::Region::new(&stats_alloc::INSTRUMENTED_SYSTEM);

    let mut state = F4State::new();

    for mut f in fs {
        if f.is_zero() {
            continue;
        }

        if opts.normalize_inputs {
            f.normalize_in_place(ctx)?;
        }

        if f.is_zero() {
            continue;
        }

        update_with_polynomial(&mut state.basis, &mut state.pending, f, criterion)?;
    }

    #[cfg(feature = "profiling-alloc")]
    {
        let stats = alloc_region.change();

        tracing::debug!(
            allocations = stats.allocations,
            deallocations = stats.deallocations,
            reallocations = stats.reallocations,
            bytes_allocated = stats.bytes_allocated,
            bytes_deallocated = stats.bytes_deallocated,
            bytes_reallocated = stats.bytes_reallocated,
            "initialize_state allocation profile"
        );
    }

    #[cfg(feature = "profile-dump")]
    {
        use crate::instrumentation::dump::dump_json;
        use crate::instrumentation::snapshot::snapshot_state;

        let snapshot = snapshot_state("initialize_state", 0, &state);

        if let Err(err) = dump_json("f4-profile", "000_initialize_state_snapshot", &snapshot) {
            tracing::warn!(error = %err, "failed to dump initialize_state snapshot");
        }
    }

    Ok(state)
}
