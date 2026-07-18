use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompareStatus {
    Match,
    Mismatch,
}

impl CompareStatus {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Match => "MATCH",
            Self::Mismatch => "MISMATCH",
        }
    }
}

#[derive(Debug, Clone)]
pub struct NormalizedPrettyCompare {
    pub status: CompareStatus,
    pub only_in_reference: Vec<String>,
    pub only_in_candidate: Vec<String>,
}

pub fn compare_normalized_pretty(reference_pretty: &[String], candidate_pretty: &[String]) -> NormalizedPrettyCompare {
    let reference: BTreeSet<_> = reference_pretty
        .iter()
        .map(|s| normalize_poly_text(s))
        .filter(|s| !s.is_empty())
        .collect();

    let candidate: BTreeSet<_> = candidate_pretty
        .iter()
        .map(|s| normalize_poly_text(s))
        .filter(|s| !s.is_empty())
        .collect();

    let only_in_reference = reference
        .difference(&candidate)
        .cloned()
        .collect::<Vec<_>>();

    let only_in_candidate = candidate
        .difference(&reference)
        .cloned()
        .collect::<Vec<_>>();

    NormalizedPrettyCompare { status: if reference == candidate { CompareStatus::Match } else { CompareStatus::Mismatch }, only_in_reference, only_in_candidate }
}

pub fn build_compare_report(case_name: &str, reference_name: &str, candidate_name: &str, reference_pretty: &[String], candidate_pretty: &[String]) -> String {
    let cmp = compare_normalized_pretty(reference_pretty, candidate_pretty);

    format!(
        "compare: {case_name}\n\
         reference: {reference_name}\n\
         candidate: {candidate_name}\n\
         normalized_pretty_status: {}\n\n\
         normalized_only_in_reference: {}\n\
         normalized_only_in_candidate: {}\n\n\
         --- only in reference ({reference_name}) ---\n{}\n\n\
         --- only in candidate ({candidate_name}) ---\n{}\n",
        cmp.status.as_str(),
        cmp.only_in_reference.len(),
        cmp.only_in_candidate.len(),
        cmp.only_in_reference.join("\n"),
        cmp.only_in_candidate.join("\n"),
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
