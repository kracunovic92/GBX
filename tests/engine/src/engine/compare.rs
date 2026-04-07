use std::collections::BTreeSet;

pub fn build_compare_report(case_name: &str, singular: &[String], gbx: &[String]) -> String {
    let a: BTreeSet<_> = singular.iter().map(|s| normalize_poly_text(s)).collect();

    let b: BTreeSet<_> = gbx.iter().map(|s| normalize_poly_text(s)).collect();

    let same = a == b;

    let only_a: Vec<_> = a.difference(&b).cloned().collect();
    let only_b: Vec<_> = b.difference(&a).cloned().collect();

    let a_sorted: Vec<_> = a.into_iter().collect();
    let b_sorted: Vec<_> = b.into_iter().collect();

    format!(
        "compare: {case_name}\nstatus: {}\n\n\
         singular_lines: {}\n\
         gbx_lines: {}\n\
         only_in_singular: {}\n\
         only_in_gbx: {}\n\n\
         --- singular (normalized) ---\n{}\n\n\
         --- gbx (normalized) ---\n{}\n\n\
         --- only in singular ---\n{}\n\n\
         --- only in gbx ---\n{}\n",
        if same { "MATCH" } else { "MISMATCH" },
        a_sorted.len(),
        b_sorted.len(),
        only_a.len(),
        only_b.len(),
        a_sorted.join("\n"),
        b_sorted.join("\n"),
        only_a.join("\n"),
        only_b.join("\n"),
    )
}

fn normalize_poly_text(s: &str) -> String {
    let mut s = s.chars().filter(|c| !c.is_whitespace()).collect::<String>();

    // Remove Singular prefix
    if let Some(rest) = s.strip_prefix("GB:") {
        s = rest.to_string();
    }

    // Remove multiplication symbols (y*z -> yz)
    s = s.replace('*', "");

    // Normalize exponent style (y^2 -> y2)
    s = s.replace('^', "");

    // Normalize signs
    s = s.replace("+-", "-");
    s = s.replace("-+", "-");
    s = s.replace("++", "+");

    s
}
