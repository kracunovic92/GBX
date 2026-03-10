use anyhow::{Context, Result};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct RunResult {
    pub ok: bool,
    pub stdout: String,
    pub stderr: String,
    pub wall_time: Duration,
}

pub fn run_singular_script(singular_bin: &str, script: &str) -> Result<RunResult> {
    let start = Instant::now();

    let mut child = Command::new(singular_bin)
        .arg("-q")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("failed to spawn Singular binary '{singular_bin}'"))?;

    {
        use std::io::Write;
        let stdin = child
            .stdin
            .as_mut()
            .context("failed to open Singular stdin")?;
        stdin
            .write_all(script.as_bytes())
            .context("failed to write Singular script to stdin")?;
    }

    let out = child
        .wait_with_output()
        .context("failed to wait for Singular")?;
    let wall = start.elapsed();

    Ok(RunResult { ok: out.status.success(), stdout: String::from_utf8_lossy(&out.stdout).into_owned(), stderr: String::from_utf8_lossy(&out.stderr).into_owned(), wall_time: wall })
}
