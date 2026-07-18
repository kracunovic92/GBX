use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "graph_experiments")]
#[command(about = "Manual testing / experimentation binary for GBX graph tooling")]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Read a DIMACS instance and print the parsed graph.
    Parse {
        /// Path to DIMACS file (e.g. instances/foo.col)
        #[arg(value_name = "FILE")]
        file: PathBuf,

        /// Normalize adjacency lists (sort + dedup) and recompute m.
        #[arg(long)]
        normalize: bool,
    },

    /// Print quick stats for a graph instance.
    Stats {
        /// Path to DIMACS file
        #[arg(value_name = "FILE")]
        file: PathBuf,

        /// Normalize adjacency lists (sort + dedup) and recompute m.
        #[arg(long)]
        normalize: bool,
    },

    /// Build k-coloring system and print encoding stats / optional polynomial dump.
    Encode {
        /// Path to DIMACS file
        #[arg(value_name = "FILE")]
        file: PathBuf,

        /// Number of colors k
        #[arg(short = 'k', long)]
        k: usize,

        /// Prime modulus p for Fp
        #[arg(long, default_value_t = 32003)]
        p: u32,

        /// Normalize adjacency lists (sort + dedup) and recompute m.
        #[arg(long)]
        normalize: bool,

        /// Dump first N polynomials (pretty format).
        #[arg(long)]
        dump: Option<usize>,
    },

    /// Encode and run Buchberger; print timings and basis dump.
    Grobner {
        /// Path to DIMACS file
        #[arg(value_name = "FILE")]
        file: PathBuf,

        /// Number of colors k
        #[arg(short = 'k', long)]
        k: usize,

        /// Prime modulus p for Fp
        #[arg(long, default_value_t = 32003)]
        p: u32,

        /// Normalize adjacency lists (sort + dedup) and recompute m.
        #[arg(long)]
        normalize: bool,

        /// Normalize inputs before starting Buchberger (recommended).
        #[arg(long, default_value_t = true)]
        normalize_inputs: bool,

        /// Normalize each remainder before inserting into basis (recommended).
        #[arg(long, default_value_t = true)]
        normalize_remainders: bool,

        /// F4 reducer backend: dense, roman, or roman-parallel.
        #[arg(long, default_value = "roman-parallel")]
        reducer: String,

        /// F4 pair batch size.
        #[arg(long, default_value_t = 64)]
        batch_size: usize,

        /// F4 basis post-processing mode: none, minimal, or reduced.
        #[arg(long, default_value = "reduced")]
        post: String,

        /// Directory for run.tsv, phases.tsv, phase_totals.tsv, and environment.txt.
        #[arg(long)]
        profile_out: Option<PathBuf>,

        /// Dump the resulting basis (pretty).
        #[arg(long)]
        dump_basis: bool,
    },
}
