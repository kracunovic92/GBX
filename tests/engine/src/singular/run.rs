use anyhow::{Context, Result};
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct RunResult {
    pub ok: bool,
    pub stdout: String,
    pub stderr: String,
    pub wall_time: Duration,

    /// Peak resident memory used by the Singular child process.
    ///
    /// On Linux this comes from `rusage.ru_maxrss`, which is reported in KiB,
    /// then converted to bytes.
    pub peak_memory_bytes: Option<u64>,
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
        let stdin = child
            .stdin
            .as_mut()
            .context("failed to open Singular stdin")?;

        stdin
            .write_all(script.as_bytes())
            .context("failed to write Singular script to stdin")?;
    }

    // Important: close stdin so Singular knows there is no more input.
    drop(child.stdin.take());

    let mut stdout_pipe = child
        .stdout
        .take()
        .context("failed to take Singular stdout")?;

    let mut stderr_pipe = child
        .stderr
        .take()
        .context("failed to take Singular stderr")?;

    let stdout_handle = std::thread::spawn(move || -> Result<String> {
        let mut stdout = String::new();
        stdout_pipe
            .read_to_string(&mut stdout)
            .context("failed to read Singular stdout")?;
        Ok(stdout)
    });

    let stderr_handle = std::thread::spawn(move || -> Result<String> {
        let mut stderr = String::new();
        stderr_pipe
            .read_to_string(&mut stderr)
            .context("failed to read Singular stderr")?;
        Ok(stderr)
    });

    let pid = child.id() as libc::pid_t;

    let mut status: libc::c_int = 0;
    let mut usage = std::mem::MaybeUninit::<libc::rusage>::uninit();

    let waited = unsafe { libc::wait4(pid, &mut status, 0, usage.as_mut_ptr()) };

    if waited < 0 {
        return Err(std::io::Error::last_os_error()).context("wait4 failed for Singular");
    }

    let usage = unsafe { usage.assume_init() };

    let wall = start.elapsed();

    let stdout = stdout_handle
        .join()
        .map_err(|_| anyhow::anyhow!("stdout reader thread panicked"))??;

    let stderr = stderr_handle
        .join()
        .map_err(|_| anyhow::anyhow!("stderr reader thread panicked"))??;

    let ok = exited_successfully(status);

    // Linux: ru_maxrss is in KiB.
    let peak_memory_bytes = Some((usage.ru_maxrss as u64) * 1024);

    Ok(RunResult { ok, stdout, stderr, wall_time: wall, peak_memory_bytes })
}

fn exited_successfully(status: libc::c_int) -> bool {
    libc::WIFEXITED(status) && libc::WEXITSTATUS(status) == 0
}
