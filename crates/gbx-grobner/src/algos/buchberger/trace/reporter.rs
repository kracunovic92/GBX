use super::snapshot::TraceSnapshot;
use super::tracer::Tracer;
use std::time::Duration;

/// Format byte counts in a compact human-readable style.
#[must_use]
pub fn fmt_bytes(n: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * 1024.0;
    const GB: f64 = 1024.0 * 1024.0 * 1024.0;

    let x = n as f64;
    if x >= GB {
        format!("{:.2} GiB", x / GB)
    } else if x >= MB {
        format!("{:.2} MiB", x / MB)
    } else if x >= KB {
        format!("{:.2} KiB", x / KB)
    } else {
        format!("{n} B")
    }
}

#[must_use]
fn fmt_duration_ms(d: Duration) -> String {
    format!("{}ms", d.as_millis())
}

pub fn print_progress_line(tracer: &Tracer, snap: &TraceSnapshot, delta: Duration, total: Duration) {
    let c = &tracer.counters;
    let wt = &tracer.while_times;

    let avg_nf_ms = if c.pops == 0 { 0.0 } else { wt.normal_form.as_secs_f64() * 1000.0 / c.pops as f64 };

    let avg_sp_ms = if c.pops == 0 { 0.0 } else { wt.s_poly.as_secs_f64() * 1000.0 / c.pops as f64 };

    eprint!(
        "[trace] pops={} gb_len={} queue_len={} max_queue_len={} pushes={} seeded_pairs={} inserted={} zero_reductions={} rejected_by_criterion={} skipped_missing_key={} unit_reductions={} avg_s_poly_ms={:.3} avg_nf_ms={:.3}",
        c.pops,
        snap.gb_len,
        snap.queue_len,
        c.max_queue_len,
        c.pushes,
        c.seeded_pairs,
        c.inserted_polys,
        c.zero_reductions,
        c.pairs_rejected_by_criterion,
        c.pairs_skipped_missing_key,
        c.unit_reductions,
        avg_sp_ms,
        avg_nf_ms,
    );

    if let Some(term_count) = snap.basis_term_count {
        eprint!(" basis_terms={term_count}");
    }
    if let Some(max_poly_terms) = snap.max_poly_terms {
        eprint!(" max_poly_terms={max_poly_terms}");
    }
    if let Some(mem) = snap.memory {
        eprint!(" rss={}", fmt_bytes(mem.rss_bytes));
    }

    eprintln!(" Δt={delta:?} total={total:?}");
}

pub fn print_summary(tracer: &Tracer, snap: &TraceSnapshot) {
    let c = &tracer.counters;
    let p = &tracer.phases;
    let w = &tracer.while_times;

    if tracer.cfg.print_phase_summary {
        eprintln!(
            "[trace] phases: init={} seed={} while={} post={}",
            fmt_duration_ms(p.init),
            fmt_duration_ms(p.seed),
            fmt_duration_ms(p.while_loop),
            fmt_duration_ms(p.post),
        );
    }

    if tracer.cfg.print_breakdown {
        eprintln!(
            "[trace] while breakdown: s_poly={} normal_form={} rem_norm={} pair_update={}",
            fmt_duration_ms(w.s_poly),
            fmt_duration_ms(w.normal_form),
            fmt_duration_ms(w.remainder_normalize),
            fmt_duration_ms(w.pair_update),
        );
    }

    eprint!(
        "[trace] totals: pops={} pushes={} seeded_pairs={} inserted_polys={} zero_reductions={} unit_reductions={} rejected_by_criterion={} skipped_missing_key={} initial_gb_len={} final_gb_len={} max_queue_len={}",
        c.pops,
        c.pushes,
        c.seeded_pairs,
        c.inserted_polys,
        c.zero_reductions,
        c.unit_reductions,
        c.pairs_rejected_by_criterion,
        c.pairs_skipped_missing_key,
        c.initial_basis_len,
        snap.gb_len,
        c.max_queue_len,
    );

    if let Some(term_count) = snap.basis_term_count {
        eprint!(" basis_terms={term_count}");
    }
    if let Some(max_poly_terms) = snap.max_poly_terms {
        eprint!(" max_poly_terms={max_poly_terms}");
    }
    if let Some(mem) = snap.memory {
        eprint!(" rss={}", fmt_bytes(mem.rss_bytes));
    }

    eprintln!();
}
