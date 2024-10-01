use anyhow::Result;
use bon::bon;
use camino::Utf8PathBuf;
use clap::Parser;
use itertools::Itertools;
use lazy_regex::Regex;
use std::{fmt::Debug, process::ExitCode};

#[derive(Parser)]
struct Args {
    pattern: String,

    /// Path to file
    input_path: Utf8PathBuf,

    /// Print number of matched lines
    #[arg(short = 'c')]
    count: bool,

    /// Print N lines after the match
    #[arg(short = 'A')]
    after: Option<usize>,

    /// Print N lines before the match
    #[arg(short = 'B')]
    before: Option<usize>,

    /// Print N lines around the match
    #[arg(short = 'C')]
    context: Option<usize>,

    /// Ignore case
    #[arg(short = 'i')]
    ignore_case: bool,

    /// Invert: instead of matching, exclude
    #[arg(short = 'v')]
    invert: bool,

    /// Search exact string, not a pattern
    #[arg(short = 'F')]
    fixed: bool,

    /// Print the lines numbers
    #[arg(short = 'n')]
    line_num: bool,
}

struct SimpleGrep {
    invert: bool,

    line_num: bool,

    regex: Regex,

    output: Output,
}

enum Output {
    CountLines,
    MatchedLines(Option<(usize, usize)>),
}

#[derive(Debug, Clone, Copy)]
enum Line<'line> {
    Matched(usize, &'line str),
    Context(usize, &'line str),
}

#[bon]
impl SimpleGrep {
    #[builder]
    fn new(
        pattern: String,
        count: bool,
        after: Option<usize>,
        before: Option<usize>,
        context: Option<usize>,
        ignore_case: bool,
        invert: bool,
        fixed: bool,
        line_num: bool,
    ) -> Result<Self> {
        let pattern = if fixed {
            lazy_regex::regex::escape(&pattern)
        } else {
            pattern
        };

        let mut regex_builder = lazy_regex::regex::RegexBuilder::new(&pattern);

        regex_builder.case_insensitive(ignore_case);

        let regex = regex_builder.build()?;

        let output = match (count, after, before, context) {
            (true, _, _, _) => Output::CountLines,
            (false, None, None, None) => Output::MatchedLines(None),
            (false, None, None, Some(num_c)) => Output::MatchedLines(Some((num_c, num_c))),
            (false, None, Some(num_b), None) => Output::MatchedLines(Some((num_b, 0))),
            (false, None, Some(num_b), Some(num_c)) => {
                Output::MatchedLines(Some((num_b.max(num_c), num_c)))
            }
            (false, Some(num_a), None, None) => Output::MatchedLines(Some((0, num_a))),
            (false, Some(num_a), None, Some(num_c)) => {
                Output::MatchedLines(Some((num_c, num_a.max(num_c))))
            }
            (false, Some(num_a), Some(num_b), None) => Output::MatchedLines(Some((num_b, num_a))),
            (false, Some(num_a), Some(num_b), Some(num_c)) => {
                Output::MatchedLines(Some((num_b.max(num_c), num_a.max(num_c))))
            }
        };

        Ok(Self {
            invert,
            line_num,
            regex,
            output,
        })
    }
}

impl SimpleGrep {
    fn process(&self, contents: &str) -> String {
        let contents = contents.lines().collect::<Vec<_>>();

        let matched_lines = self.grep(&contents);

        let result = match self.output {
            Output::CountLines => matched_lines.len().to_string(),
            Output::MatchedLines(Some((num_before, num_after))) => {
                let lines =
                    Self::process_context_lines(&contents, &matched_lines, num_before, num_after);

                self.prepare_lines(&lines)
            }
            Output::MatchedLines(None) => {
                let lines = matched_lines
                    .into_iter()
                    .map(|(line_num, line)| Line::Matched(line_num, line))
                    .collect::<Vec<_>>();
                self.prepare_lines(&lines)
            }
        };

        result
    }

    fn grep<'line>(&self, contents: &[&'line str]) -> Vec<(usize, &'line str)> {
        contents
            .iter()
            .enumerate()
            .filter_map(|(line_num, &line)| {
                let is_match = self.regex.is_match(line);
                if is_match ^ self.invert {
                    Some((line_num, line))
                } else {
                    None
                }
            })
            .collect()
    }

    fn prepare_lines(&self, matched_lines: &[Line<'_>]) -> String {
        matched_lines
            .iter()
            .copied()
            .map(|line| match line {
                Line::Matched(line_num, line) if self.line_num => {
                    format!("{}:{line}", line_num + 1)
                }
                Line::Context(line_num, line) if self.line_num => {
                    format!("{}-{line}", line_num + 1)
                }
                Line::Matched(_, line) | Line::Context(_, line) => line.to_owned(),
            })
            .join("\n")
    }

    fn process_context_lines<'line>(
        contents: &[&'line str],
        matched_lines: &[(usize, &str)],
        num_before: usize,
        num_after: usize,
    ) -> Vec<Line<'line>> {
        let x = matched_lines
            .iter()
            .copied()
            .flat_map(|(line_num, _)| {
                let start = if num_before > line_num {
                    0
                } else {
                    line_num - num_before
                };

                let end = if line_num + num_after + 1 > contents.len() {
                    contents.len() - 1
                } else {
                    line_num + num_after
                };

                start..=end
            })
            .unique()
            .collect::<Vec<_>>();

        x.iter()
            .map(|&line_num| {
                let line = *contents.get(line_num).expect("line exist");
                match matched_lines.binary_search_by(|(probe, _)| probe.cmp(&line_num)) {
                    Ok(_) => Line::Matched(line_num, line),
                    Err(_) => Line::Context(line_num, line),
                }
            })
            .collect()
    }
}

fn run(args: Args) -> Result<()> {
    let contents = std::fs::read_to_string(args.input_path)?;

    let simple_grep = SimpleGrep::builder()
        .pattern(args.pattern)
        .count(args.count)
        .maybe_after(args.after)
        .maybe_before(args.before)
        .maybe_context(args.context)
        .ignore_case(args.ignore_case)
        .invert(args.invert)
        .fixed(args.fixed)
        .line_num(args.line_num)
        .build()?;

    let matches = simple_grep.process(&contents);

    println!("{matches}");

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
