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

pub fn extract_time_ms(stdout: &str) -> Option<u128> {
    stdout.lines().find_map(|line| {
        let line = line.trim();
        let rest = line.strip_prefix("TIME_MS:")?;
        rest.trim().parse::<u128>().ok()
    })
}
