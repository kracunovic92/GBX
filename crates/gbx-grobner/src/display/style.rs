/// Formatting style for Gröbner basis display.
///
/// Controls how each polynomial in the basis is rendered.
#[derive(Debug, Clone, Copy)]
pub enum GbStyle {
    /// Human: each polynomial on its own line
    PrettyLines,
    /// Stable dumps: each polynomial as tuple dump on its own line
    TupleLines,
}
