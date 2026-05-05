pub fn current_rss_bytes() -> Option<u64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;

    for line in status.lines() {
        let Some(rest) = line.strip_prefix("VmRSS:") else {
            continue;
        };

        let kb = rest.split_whitespace().next()?.parse::<u64>().ok()?;

        return Some(kb * 1024);
    }

    None
}
