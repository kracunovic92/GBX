/// Formatting style for Gröbner basis display.
#[derive(Debug, Clone, Copy)]
pub enum GbStyle {
    /// Human-readable form: one polynomial per line.
    PrettyLines,

    /// Stable tuple dump: one polynomial per line.
    TupleLines,
}
