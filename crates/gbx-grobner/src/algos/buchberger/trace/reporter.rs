use super::snapshot::BuchbergerTraceSnapshot;
use super::tracer::BuchbergerTracer;
use crate::trace::{fmt_bytes, fmt_duration_ms};
use std::time::Duration;

/// Formats a periodic Buchberger progress line.
#[must_use]
pub fn format_progress_line(tracer: &BuchbergerTracer, snap: &BuchbergerTraceSnapshot, delta: Duration, total: Duration) -> String {
    let c = tracer.counters();
    let m = &c.mech;
    let wt = tracer.while_times();

    let avg_nf_ms = if m.pops == 0 { 0.0 } else { wt.normal_form.as_secs_f64() * 1000.0 / m.pops as f64 };

    let avg_sp_ms = if m.pops == 0 { 0.0 } else { wt.s_poly.as_secs_f64() * 1000.0 / m.pops as f64 };

    let mut out = format!(
        "[trace] pops={} gb_len={} queue_len={} max_queue_len={} pushes={} seeded_pairs={} inserted={} zero_reductions={} rejected_by_criterion={} rejected_by_filter={} skipped_missing_key={} pairs_added_by_update={} unit_reductions={} avg_s_poly_ms={:.3} avg_nf_ms={:.3}",
        m.pops,
        snap.gb_len,
        snap.queue_len,
        m.max_queue_len,
        m.pushes,
        m.seeded_pairs,
        c.inserted_polys,
        c.zero_reductions,
        m.pairs_rejected_by_criterion,
        m.pairs_rejected_by_filter,
        m.pairs_skipped_missing_key,
        m.pairs_added_by_update,
        c.unit_reductions,
        avg_sp_ms,
        avg_nf_ms,
    );

    if let Some(term_count) = snap.basis_term_count {
        out.push_str(&format!(" basis_terms={term_count}"));
    }
    if let Some(max_poly_terms) = snap.max_poly_terms {
        out.push_str(&format!(" max_poly_terms={max_poly_terms}"));
    }
    if let Some(mem) = snap.memory {
        out.push_str(&format!(" rss={}", fmt_bytes(mem.rss_bytes)));
    }

    out.push_str(&format!(" Δt={delta:?} total={total:?}"));
    out
}

/// Formats a per-iteration Buchberger line.
#[must_use]
pub fn format_iteration_line(
    tracer: &BuchbergerTracer,
    i: usize,
    j: usize,
    gb_len: usize,
    queue_len: usize,
    s_poly_dt: Duration,
    nf_dt: Duration,
    rem_norm_dt: Duration,
    pair_update_dt: Duration,
    outcome: &str,
) -> String {
    let pops = tracer.counters().mech.pops;

    if tracer.cfg().print_iteration_timings {
        format!(
            "[trace] iter pop={} pair=({}, {}) gb_len={} queue_len={} outcome={} s_poly={:?} nf={:?} rem_norm={:?} pair_update={:?}",
            pops, i, j, gb_len, queue_len, outcome, s_poly_dt, nf_dt, rem_norm_dt, pair_update_dt,
        )
    } else {
        format!(
            "[trace] iter pop={} pair=({}, {}) gb_len={} queue_len={} outcome={}",
            pops, i, j, gb_len, queue_len, outcome,
        )
    }
}

/// Formats the final Buchberger summary as multiple lines.
#[must_use]
pub fn format_summary_lines(tracer: &BuchbergerTracer, snap: &BuchbergerTraceSnapshot) -> Vec<String> {
    let mut lines = Vec::new();

    let c = tracer.counters();
    let m = &c.mech;
    let p = tracer.phases();
    let w = tracer.while_times();
    let cfg = tracer.cfg();

    if cfg.print_phase_summary {
        lines.push(format!(
            "[trace] phases: init={} seed={} while={} post={}",
            fmt_duration_ms(p.init),
            fmt_duration_ms(p.seed),
            fmt_duration_ms(p.while_loop),
            fmt_duration_ms(p.post),
        ));
    }

    if cfg.print_breakdown {
        lines.push(format!(
            "[trace] while breakdown: s_poly={} normal_form={} rem_norm={} pair_update={}",
            fmt_duration_ms(w.s_poly),
            fmt_duration_ms(w.normal_form),
            fmt_duration_ms(w.remainder_normalize),
            fmt_duration_ms(w.pair_update),
        ));
    }

    let mut totals = format!(
        "[trace] totals: pops={} pushes={} seeded_pairs={} inserted_polys={} zero_reductions={} unit_reductions={} rejected_by_criterion={} rejected_by_filter={} skipped_missing_key={} pairs_added_by_update={} initial_gb_len={} final_gb_len={} max_queue_len={}",
        m.pops,
        m.pushes,
        m.seeded_pairs,
        c.inserted_polys,
        c.zero_reductions,
        c.unit_reductions,
        m.pairs_rejected_by_criterion,
        m.pairs_rejected_by_filter,
        m.pairs_skipped_missing_key,
        m.pairs_added_by_update,
        m.initial_basis_len,
        snap.gb_len,
        m.max_queue_len,
    );

    if let Some(term_count) = snap.basis_term_count {
        totals.push_str(&format!(" basis_terms={term_count}"));
    }
    if let Some(max_poly_terms) = snap.max_poly_terms {
        totals.push_str(&format!(" max_poly_terms={max_poly_terms}"));
    }
    if let Some(mem) = snap.memory {
        totals.push_str(&format!(" rss={}", fmt_bytes(mem.rss_bytes)));
    }

    lines.push(totals);
    lines
}

/// Prints a periodic progress line.
pub fn print_progress_line(tracer: &BuchbergerTracer, snap: &BuchbergerTraceSnapshot, delta: Duration, total: Duration) {
    eprintln!("{}", format_progress_line(tracer, snap, delta, total));
}

/// Prints a per-iteration line.
pub fn print_iteration_line(
    tracer: &BuchbergerTracer,
    i: usize,
    j: usize,
    gb_len: usize,
    queue_len: usize,
    s_poly_dt: Duration,
    nf_dt: Duration,
    rem_norm_dt: Duration,
    pair_update_dt: Duration,
    outcome: &str,
) {
    eprintln!(
        "{}",
        format_iteration_line(
            tracer,
            i,
            j,
            gb_len,
            queue_len,
            s_poly_dt,
            nf_dt,
            rem_norm_dt,
            pair_update_dt,
            outcome,
        )
    );
}

/// Prints the final summary.
pub fn print_summary(tracer: &BuchbergerTracer, snap: &BuchbergerTraceSnapshot) {
    for line in format_summary_lines(tracer, snap) {
        eprintln!("{line}");
    }
}
