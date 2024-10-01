use std::{io::BufRead, process::ExitCode};

use anyhow::Result;
use clap::Parser;
use itertools::Itertools;

/// Utility similar to the console command `cut`
#[derive(Debug, Parser)]
struct Args {
    /// Select fields (columns). Provide columns numbers, separated by commas.
    #[arg(short, long, required = true)]
    fields: String,

    /// Use a different separator
    #[arg(short, long, default_value = "\t")]
    delimeter: String,

    /// Only lines with a separator
    #[arg(short, long)]
    separated: bool,
}

fn run(args: Args) -> Result<()> {
    let fields = args
        .fields
        .split(',')
        .map(|number| number.parse::<usize>().map_err(|err| anyhow::anyhow!(err)))
        .collect::<Result<Vec<usize>>>()?;

    let delimeter = args.delimeter;
    let separated = args.separated;

    let stdin = std::io::stdin().lock();

    let lines = stdin.lines().process_results(|iter| {
        iter.map(|line| {
            let line = line.trim().to_owned();

            if separated && !line.contains(&delimeter) {
                return String::new();
            }

            line.split(&delimeter)
                .enumerate()
                .filter(|&(index, _)| fields.contains(&(index + 1)))
                .map(|(_, column)| column)
                .join(&delimeter)
        })
        .collect::<Vec<_>>()
    })?;

    for line in lines {
        println!("{line}");
    }

    Ok(())
}

fn main() -> ExitCode {
    let args = Args::parse();

    if let Err(err) = run(args) {
        eprintln!("{err:?}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
