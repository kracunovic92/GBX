/// Writes a Gröbner basis in human-readable form into a formatter.
///
/// This macro formats the given basis using
/// `GbDisplay::pretty_lines` and writes the result into `dst`,
/// which must implement [`core::fmt::Write`].
///
/// # Arguments
///
/// - `$dst`: destination implementing `fmt::Write`
/// - `$ring`: ring context
/// - `$basis`: Gröbner basis
/// - `$vars`: variable names (in ring order)
///
/// # Example
///
/// ```ignore
/// let mut s = String::new();
/// write_gb_pretty!(s, &ring, &basis, &vars).unwrap();
/// println!("{s}");
/// ```
///
/// # Output Format
///
/// Each polynomial is printed on its own line in mathematical form.
#[macro_export]
macro_rules! write_gb_pretty {
    ($dst:expr, $ring:expr, $basis:expr, $vars:expr) => {{
        use core::fmt::Write as _;
        let disp = $crate::display::GbDisplay::pretty_lines($ring, $basis, $vars);
        ($dst).write_fmt(format_args!("{}", disp))
    }};
}

/// Writes a Gröbner basis in stable tuple format into a formatter.
///
/// This macro formats the basis using
/// `GbDisplay::tuple_lines` and writes the result into `dst`.
///
/// # Purpose
///
/// Intended for:
///
/// - Snapshot testing
/// - Diff-based comparison
/// - External CAS cross-checking (e.g. Singular)
///
/// # Output Format
///
/// Each polynomial is printed on its own line,
/// using tuple representation:
///
/// ```text
/// [(c,[...]),(c,[...])]
/// ```
///
/// # Example
///
/// ```ignore
/// let mut s = String::new();
/// write_gb_tuple!(s, &ring, &basis, &vars).unwrap();
/// ```
#[macro_export]
macro_rules! write_gb_tuple {
    ($dst:expr, $ring:expr, $basis:expr, $vars:expr) => {{
        use core::fmt::Write as _;
        let disp = $crate::display::GbDisplay::tuple_lines($ring, $basis, $vars);
        ($dst).write_fmt(format_args!("{}", disp))
    }};
}

/// Returns a human-readable string representation of a Gröbner basis.
///
/// Equivalent to calling:
///
/// ```ignore
/// GbDisplay::pretty_lines(ring, basis, vars).to_string()
/// ```
///
/// # Example
///
/// ```ignore
/// let s = gb_pretty_str!(&ring, &basis, &vars);
/// println!("{s}");
/// ```
#[macro_export]
macro_rules! gb_pretty_str {
    ($ring:expr, $basis:expr, $vars:expr) => {{ $crate::grobner::display::GbDisplay::pretty_lines($ring, $basis, $vars).to_string() }};
}

/// Returns a stable tuple-string representation of a Gröbner basis.
///
/// Intended for deterministic comparison and testing.
///
/// Equivalent to:
///
/// ```ignore
/// GbDisplay::tuple_lines(ring, basis, vars).to_string()
/// ```
///
/// # Example
///
/// ```ignore
/// let s = gb_tuple_str!(&ring, &basis, &vars);
/// assert!(s.contains("[("));
/// ```
#[macro_export]
macro_rules! gb_tuple_str {
    ($ring:expr, $basis:expr, $vars:expr) => {{ $crate::grobner::display::GbDisplay::tuple_lines($ring, $basis, $vars).to_string() }};
}
