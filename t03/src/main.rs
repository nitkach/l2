use anyhow::Result;
use camino::Utf8PathBuf;
use clap::Parser;
use std::process::ExitCode;

mod months;
mod sort;

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

    #[clap(flatten)]
    sort_flags: SortFlags,

    /// Sort in reverse order
    #[arg(short = 'r')]
    reverse: bool,

    /// Do not output dublicates
    #[arg(short = 'u')]
    unique: bool,

    /// Ignore trailing spaces
    #[arg(short = 'b')]
    ignore_trailing_spaces: bool,

    /// Check if the data is already sorted
    #[arg(short = 'c')]
    check_sorted: bool,

    #[arg(long = "sep", default_value = " ")]
    separator: String,
}

#[derive(Parser, Debug)]
#[group(multiple = false)]
struct SortFlags {
    /// Sort numbers
    #[arg(short = 'n')]
    sort_numbers: bool,

    /// Sort by month
    #[arg(short = 'M')]
    sort_month: bool,

    /// Sort by numeric value taking into account suffixes
    #[arg(short = 's')]
    sort_numbers_with_suffixes: bool,
}

fn run(args: Args) -> Result<()> {
    let contents = std::fs::read_to_string(args.input_path)?;

    let sort = sort::Sort::builder()
        .sort_column(args.sort_column)
        .by_numbers(args.sort_flags.sort_numbers)
        .by_numbers_with_suffixes(args.sort_flags.sort_numbers_with_suffixes)
        .by_month(args.sort_flags.sort_month)
        .reverse(args.reverse)
        .unique(args.unique)
        .ignore_trailing_spaces(args.ignore_trailing_spaces)
        .check_sorted(args.check_sorted)
        .separator(args.separator)
        .build();

    if let Some(sorted) = sort.check_is_sorted(&contents)? {
        if sorted {
            println!("Sorted");
        } else {
            println!("Not sorted");
        }

        return Ok(());
    }

    let sorted = sort.sort_contents(&contents)?;
    let sorted_contents = sorted.join("\n");

    std::fs::write(args.output_path, sorted_contents)?;

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
