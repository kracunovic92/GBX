/// Writes a Gröbner basis in human-readable form into a formatter.
///
/// `$basis` must be a `GrobnerBasis<P>` or anything exposing `.as_slice()`.
#[macro_export]
macro_rules! write_gb_pretty {
    ($dst:expr, $ring:expr, $basis:expr, $vars:expr) => {{
        use core::fmt::Write as _;

        let disp = $crate::display::GbDisplay::pretty_lines($ring, ($basis).as_slice(), $vars);

        ($dst).write_fmt(format_args!("{}", disp))
    }};
}

/// Writes a Gröbner basis in stable tuple format into a formatter.
///
/// `$basis` must be a `GrobnerBasis<P>` or anything exposing `.as_slice()`.
#[macro_export]
macro_rules! write_gb_tuple {
    ($dst:expr, $ring:expr, $basis:expr, $vars:expr) => {{
        use core::fmt::Write as _;

        let disp = $crate::display::GbDisplay::tuple_lines($ring, ($basis).as_slice(), $vars);

        ($dst).write_fmt(format_args!("{}", disp))
    }};
}

/// Returns a human-readable string representation of a Gröbner basis.
#[macro_export]
macro_rules! gb_pretty_str {
    ($ring:expr, $basis:expr, $vars:expr) => {{ $crate::display::GbDisplay::pretty_lines($ring, ($basis).as_slice(), $vars).to_string() }};
}

/// Returns a stable tuple-string representation of a Gröbner basis.
#[macro_export]
macro_rules! gb_tuple_str {
    ($ring:expr, $basis:expr, $vars:expr) => {{ $crate::display::GbDisplay::tuple_lines($ring, ($basis).as_slice(), $vars).to_string() }};
}
