pub fn extract_gb_lines(stdout: &str) -> Vec<String> {
    stdout
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}
