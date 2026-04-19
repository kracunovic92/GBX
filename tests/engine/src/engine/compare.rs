use std::collections::BTreeSet;

pub fn build_compare_report(case_name: &str, singular_canonical: &[String], gbx_canonical: &[String], singular_pretty: &[String], gbx_pretty: &[String]) -> String {
    let a_can: BTreeSet<_> = singular_canonical.iter().cloned().collect();
    let b_can: BTreeSet<_> = gbx_canonical.iter().cloned().collect();

    let can_same = a_can == b_can;
    let can_only_a: Vec<_> = a_can.difference(&b_can).cloned().collect();
    let can_only_b: Vec<_> = b_can.difference(&a_can).cloned().collect();

    let a_pretty: BTreeSet<_> = singular_pretty
        .iter()
        .map(|s| normalize_poly_text(s))
        .collect();
    let b_pretty: BTreeSet<_> = gbx_pretty.iter().map(|s| normalize_poly_text(s)).collect();

    let pretty_same = a_pretty == b_pretty;
    let pretty_only_a: Vec<_> = a_pretty.difference(&b_pretty).cloned().collect();
    let pretty_only_b: Vec<_> = b_pretty.difference(&a_pretty).cloned().collect();

    format!(
        "compare: {case_name}\n\
         canonical_status: {}\n\
         normalized_pretty_status: {}\n\n\
         canonical_only_in_singular: {}\n\
         canonical_only_in_gbx: {}\n\
         normalized_only_in_singular: {}\n\
         normalized_only_in_gbx: {}\n\n\
         --- only in singular (canonical) ---\n{}\n\n\
         --- only in gbx (canonical) ---\n{}\n\n\
         --- only in singular (normalized pretty) ---\n{}\n\n\
         --- only in gbx (normalized pretty) ---\n{}\n",
        if can_same { "MATCH" } else { "MISMATCH" },
        if pretty_same { "MATCH" } else { "MISMATCH" },
        can_only_a.len(),
        can_only_b.len(),
        pretty_only_a.len(),
        pretty_only_b.len(),
        can_only_a.join("\n"),
        can_only_b.join("\n"),
        pretty_only_a.join("\n"),
        pretty_only_b.join("\n"),
    )
}

fn normalize_poly_text(s: &str) -> String {
    let mut s = s.chars().filter(|c| !c.is_whitespace()).collect::<String>();

    if let Some(rest) = s.strip_prefix("GB:") {
        s = rest.to_string();
    }

    s = s.replace('*', "");
    s = s.replace('^', "");

    s = s.replace("+-", "-");
    s = s.replace("-+", "-");
    s = s.replace("++", "+");
    s = s.replace("--", "+");

    s
}
