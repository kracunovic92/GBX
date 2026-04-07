pub fn extract_gb_lines(stdout: &str) -> Vec<String> {
    stdout
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            let rest = line.strip_prefix("GB:")?;
            let poly = rest.trim();
            if poly.is_empty() { None } else { Some(poly.to_string()) }
        })
        .collect()
}
