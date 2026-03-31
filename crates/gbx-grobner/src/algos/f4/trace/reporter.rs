use super::snapshot::F4TraceSnapshot;
use super::tracer::F4Tracer;
use crate::trace::{fmt_bytes, fmt_duration_ms};
use std::time::Duration;

#[must_use]
fn pct(part: Duration, total: Duration) -> f64 {
    let total_secs = total.as_secs_f64();
    if total_secs == 0.0 { 0.0 } else { 100.0 * part.as_secs_f64() / total_secs }
}

#[must_use]
fn avg_ms(dt: Duration, n: u64) -> f64 {
    if n == 0 { 0.0 } else { (dt.as_secs_f64() * 1000.0) / n as f64 }
}

/// Formats a periodic F4 progress line.
#[must_use]
pub fn format_progress_line(tracer: &F4Tracer, snap: &F4TraceSnapshot, delta: Duration, total: Duration) -> String {
    let c = tracer.counters();
    let m = &c.mech;
    let w = tracer.while_times();
    let while_total = w.total();

    let mut out = format!(
        "[trace] batches={} selected_pairs={} gb_len={} queue_len={} max_queue_len={} pushes={} pops={} rejected_by_criterion={} rejected_by_filter={} skipped_missing_key={} pairs_added_by_update={} seed_rows={} symbolic_rows={} matrix_rows={} matrix_cols={} extracted={} inserted={} skipped_zero_extracted={}",
        c.selected_batches,
        c.selected_pairs,
        snap.gb_len,
        snap.queue_len,
        m.max_queue_len,
        m.pushes,
        m.pops,
        m.pairs_rejected_by_criterion,
        m.pairs_rejected_by_filter,
        m.pairs_skipped_missing_key,
        m.pairs_added_by_update,
        c.seed_rows_total,
        c.symbolic_rows_total,
        c.matrix_rows_total,
        c.matrix_cols_total,
        c.extracted_total,
        c.inserted_total,
        c.skipped_zero_extracted_total,
    );

    if let Some(mem) = snap.memory {
        out.push_str(&format!(" rss={}", fmt_bytes(mem.rss_bytes)));
    }

    if while_total > Duration::ZERO {
        out.push_str(&format!(
            " reduce={} ({:.1}%) build={} ({:.1}%) symbolic={} ({:.1}%)",
            fmt_duration_ms(w.row_reduce),
            pct(w.row_reduce, while_total),
            fmt_duration_ms(w.matrix_build),
            pct(w.matrix_build, while_total),
            fmt_duration_ms(w.symbolic),
            pct(w.symbolic, while_total),
        ));
    }

    out.push_str(&format!(
        " Δt={} total={}",
        fmt_duration_ms(delta),
        fmt_duration_ms(total),
    ));
    out
}

/// Formats a per-batch F4 line.
#[must_use]
pub fn format_batch_line(
    tracer: &F4Tracer,
    batch_len: usize,
    gb_len: usize,
    queue_len: usize,
    batch_select_dt: Duration,
    seed_rows_dt: Duration,
    symbolic_dt: Duration,
    matrix_build_dt: Duration,
    row_reduce_dt: Duration,
    extract_dt: Duration,
    pair_update_dt: Duration,
    outcome: &str,
) -> String {
    let batch_no = tracer.counters().selected_batches;

    if tracer.cfg().print_batch_timings {
        format!(
            "[trace] batch={} pairs={} gb_len={} queue_len={} outcome={} select={} seed_rows={} symbolic={} matrix_build={} row_reduce={} extract={} pair_update={}",
            batch_no,
            batch_len,
            gb_len,
            queue_len,
            outcome,
            fmt_duration_ms(batch_select_dt),
            fmt_duration_ms(seed_rows_dt),
            fmt_duration_ms(symbolic_dt),
            fmt_duration_ms(matrix_build_dt),
            fmt_duration_ms(row_reduce_dt),
            fmt_duration_ms(extract_dt),
            fmt_duration_ms(pair_update_dt),
        )
    } else {
        format!(
            "[trace] batch={} pairs={} gb_len={} queue_len={} outcome={}",
            batch_no, batch_len, gb_len, queue_len, outcome,
        )
    }
}

/// Formats the final F4 summary as multiple lines.
#[must_use]
pub fn format_summary_lines(tracer: &F4Tracer, snap: &F4TraceSnapshot) -> Vec<String> {
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
        let while_total = w.total();
        let batches = c.selected_batches;

        lines.push(format!(
            "[trace] while total={}",
            fmt_duration_ms(while_total),
        ));

        for (name, dt) in [
            ("batch_select", w.batch_select),
            ("seed_rows", w.seed_rows),
            ("symbolic", w.symbolic),
            ("matrix_build", w.matrix_build),
            ("row_reduce", w.row_reduce),
            ("extract", w.extract),
            ("pair_update", w.pair_update),
        ] {
            lines.push(format!(
                "[trace] while {name}: total={} pct={:.2}% avg_per_batch={:.3}ms",
                fmt_duration_ms(dt),
                pct(dt, while_total),
                avg_ms(dt, batches),
            ));
        }
    }

    let mut totals = format!(
        "[trace] totals: batches={} selected_pairs={} seed_rows={} symbolic_reducer_rows={} symbolic_rows={} matrix_rows={} matrix_cols={} extracted={} inserted={} skipped_zero_extracted={} pushes={} pops={} rejected_by_criterion={} rejected_by_filter={} skipped_missing_key={} pairs_added_by_update={} initial_gb_len={} final_gb_len={} max_queue_len={}",
        c.selected_batches,
        c.selected_pairs,
        c.seed_rows_total,
        c.symbolic_reducer_rows_total,
        c.symbolic_rows_total,
        c.matrix_rows_total,
        c.matrix_cols_total,
        c.extracted_total,
        c.inserted_total,
        c.skipped_zero_extracted_total,
        m.pushes,
        m.pops,
        m.pairs_rejected_by_criterion,
        m.pairs_rejected_by_filter,
        m.pairs_skipped_missing_key,
        m.pairs_added_by_update,
        m.initial_basis_len,
        snap.gb_len,
        m.max_queue_len,
    );

    if let Some(mem) = snap.memory {
        totals.push_str(&format!(" rss={}", fmt_bytes(mem.rss_bytes)));
    }

    lines.push(totals);
    lines
}

pub fn print_progress_line(tracer: &F4Tracer, snap: &F4TraceSnapshot, delta: Duration, total: Duration) {
    eprintln!("{}", format_progress_line(tracer, snap, delta, total));
}

pub fn print_batch_line(
    tracer: &F4Tracer,
    batch_len: usize,
    gb_len: usize,
    queue_len: usize,
    batch_select_dt: Duration,
    seed_rows_dt: Duration,
    symbolic_dt: Duration,
    matrix_build_dt: Duration,
    row_reduce_dt: Duration,
    extract_dt: Duration,
    pair_update_dt: Duration,
    outcome: &str,
) {
    eprintln!(
        "{}",
        format_batch_line(
            tracer,
            batch_len,
            gb_len,
            queue_len,
            batch_select_dt,
            seed_rows_dt,
            symbolic_dt,
            matrix_build_dt,
            row_reduce_dt,
            extract_dt,
            pair_update_dt,
            outcome,
        )
    );
}

pub fn print_summary(tracer: &F4Tracer, snap: &F4TraceSnapshot) {
    for line in format_summary_lines(tracer, snap) {
        eprintln!("{line}");
    }
}
