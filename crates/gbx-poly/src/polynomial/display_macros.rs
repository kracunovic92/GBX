/// Write a polynomial in human pretty form into a `fmt::Write` target.
///
/// Usage inside Display:
/// ```
/// write_pretty!(f, ring, poly, vars)
/// ```
///
/// Usage into String:
/// ```
/// let mut s = String::new();
/// write_pretty!(&mut s, ring, poly, vars).unwrap();
/// ```
#[macro_export]
macro_rules! write_pretty {
    ($dst:expr, $ring:expr, $poly:expr, $vars:expr) => {{
        use core::fmt::Write as _;
        let disp = $crate::polynomial::PolyDisplay::pretty($ring, $poly, $vars);
        ($dst).write_fmt(format_args!("{}", disp))
    }};
}

/// Write a polynomial as a stable tuple dump into a `fmt::Write` target.
///
/// Usage:
/// ```
/// write_tuple_dump!(f, ring, poly)
/// ```
#[macro_export]
macro_rules! tuple_dump_str {
    ($ring:expr, $p:expr) => {{
        let empty: &[String] = &[];
        let disp = $crate::polynomial::PolyDisplay::tuple_dump($ring, $p, empty);
        disp.to_string()
    }};
}

/// Convenience: return a pretty polynomial as a `String`.
#[macro_export]
macro_rules! pretty_str {
    ($ring:expr, $poly:expr, $vars:expr) => {{
        let disp = $crate::polynomial::PolyDisplay::pretty($ring, $poly, $vars);
        disp.to_string()
    }};
}
