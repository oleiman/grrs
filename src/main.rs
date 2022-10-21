#![allow(unused)]

use clap::Parser;
use std::io::{BufReader, BufRead, Read};
use std::io::{Write, BufWriter};
use anyhow::{Context, Result};
use log::{info, warn};

/// Search for a pattern in a file and display the lines that contain it
#[derive(Parser, Debug)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
    // TODO(oren): how to pass this into env_logger?
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn main() -> Result<()> {
    env_logger::init();
    info!("starting up");
    let args = Cli::parse();
    let mut stdout = BufWriter::new(std::io::stdout().lock());
    writeln!(stdout, "{:?}", args);

    let f = std::fs::File::open(&args.path)
        .with_context(|| format!("Could not read file `{}`", args.path.display()))?;
    let mut reader = BufReader::new(f);
    let count = grrs::find_matches(&mut reader, &args.pattern, &mut stdout)?;
    writeln!(stdout, "Found {} lines", count);
    stdout.flush();
    Ok(())

    // NOTE(oren): progress bar example
    // let mut i = 1;
    // let pb = indicatif::ProgressBar::new(10);
    // for i in i..10 {
    //     pb.inc(1);
    // }
    // pb.finish_with_message("done");
}

