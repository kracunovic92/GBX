use crate::cli::{CliGbxReducerKind, RunGbxOneCli};
use crate::gbx::config::{GbxConfig, GbxReducerKind};
use crate::utils::test_file_config::TestFile;
use anyhow::{bail, Context, Result};

pub fn run_gbx_one(args: RunGbxOneCli) -> Result<()> {
    let file = TestFile::from_path(&args.cases).with_context(|| format!("loading cases from {}", args.cases.display()))?;

    let Some(case) = file.cases.iter().find(|c| c.name == args.case_name) else {
        bail!("case '{}' not found", args.case_name);
    };

    let reducer = match args.reducer {
        CliGbxReducerKind::Dense => GbxReducerKind::Dense,
        CliGbxReducerKind::Roman => GbxReducerKind::Roman,
        CliGbxReducerKind::RomanParallel => GbxReducerKind::RomanParallel,
        CliGbxReducerKind::All => bail!("run-gbx-one requires a concrete reducer, not all"),
    };

    let cfg = GbxConfig::new(reducer);

    let start = std::time::Instant::now();
    let out = crate::gbx::adapter::gbx_compute_basis(case, &cfg)?;
    let compute_time_ms = start.elapsed().as_millis();

    println!("TIME_MS:{compute_time_ms}");

    for line in out.basis_pretty_lines {
        println!("GB:{line}");
    }

    Ok(())
}
