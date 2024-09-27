use std::process::ExitCode;

use anyhow::{anyhow, Context, Result};
use camino::Utf8PathBuf;
use clap::Parser;
use itertools::Itertools;

/// Utility for sorting strings in file
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Parser)]
struct Args {
    /// Path to the input file for sorting
    input_path: Utf8PathBuf,

    /// Path to the output sorted file
    output_path: Utf8PathBuf,

    /// Column number to sort by
    sort_column: usize,

    /// Sort numbers
    #[arg(short = 'n')]
    sort_numbers: bool,

    /// Sort in reverse order
    #[arg(short = 'r')]
    reverse: bool,

    /// Do not output dublicates
    #[arg(short = 'u')]
    unique: bool,

    /// Sort by month
    #[arg(short = 'M')]
    month: bool,

    /// Ignore trailing spaces
    #[arg(short = 'b')]
    trailing_spaces: bool,

    /// Check if the data is already sorted
    #[arg(short = 'c')]
    check_sorted: bool,

    /// Sort by numeric value taking into account suffixes
    #[arg(short = 's')]
    suffixes: bool,

    #[arg(long = "sep", default_value = " ")]
    separator: String,
}

fn sort<O: Ord, L>(sort: &mut [(O, L)], reverse: bool) {
    sort.sort_unstable_by(|(a, _), (b, _)| {
        let cmp = a.cmp(b);

        if reverse {
            cmp.reverse()
        } else {
            cmp
        }
    });
}

fn run(args: Args) -> Result<()> {
    let contents = std::fs::read_to_string(args.input_path)?;

    let unique = args.unique;
    let split_at = args.separator;
    let sort_column = args.sort_column;
    let reverse = args.reverse;

    let lines = if unique {
        contents.lines().unique().collect::<Vec<_>>()
    } else {
        contents.lines().collect()
    };

    dbg!(&lines);

    let lines: String = if args.sort_numbers {
        sort_by_numbers(&lines, &split_at, sort_column, reverse)?
    } else {
        sort_by_str(lines, &split_at, sort_column, reverse)?
    };

    std::fs::write(args.output_path, lines)?;

    Ok(())
}

fn sort_by_str(
    lines: Vec<&str>,
    split_at: &str,
    sort_column: usize,
    reverse: bool,
) -> Result<String, anyhow::Error> {
    let mut lines = lines
        .into_iter()
        .map(|line| {
            let nth = line
                .trim_end()
                .split(&split_at)
                .nth(sort_column - 1)
                .ok_or_else(|| anyhow!("cannot find specified column: {sort_column}"))?;
            Ok((nth, line.trim_end()))
        })
        .collect::<Result<Vec<_>>>()?;
    dbg!(&lines);
    sort(&mut lines, reverse);
    Ok(lines.into_iter().map(|(_, line)| line).join("\n"))
}

fn sort_by_numbers(
    lines: &[&str],
    split_at: &str,
    sort_column: usize,
    reverse: bool,
) -> Result<String, anyhow::Error> {
    let mut lines = lines
        .iter()
        .map(|line| {
            let nth = dbg!(line
                .trim_end()
                .split(&split_at)
                .nth(sort_column - 1)
                .ok_or_else(|| anyhow!("cannot find specified column: {sort_column}"))?);
            let number = nth.parse::<i64>().with_context(|| {
                format!("Column {sort_column} doesn't contain only numbers: \"{nth}\"")
            })?;

            Ok((number, line.trim_end()))
        })
        .collect::<Result<Vec<_>>>()?;
    dbg!(&lines);
    sort(&mut lines, reverse);
    Ok(lines.into_iter().map(|(_, line)| line).join("\n"))
}

fn main() -> ExitCode {
    let args = dbg!(Args::parse());

    if let Err(err) = run(args) {
        eprintln!("{err:?}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
