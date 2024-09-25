use anyhow::Result;
use camino::Utf8PathBuf;
use clap::Parser;

#[derive(Debug, Parser)]
/// Utility for counting the number of objects in a file.
struct Args {
    /// Path to the text file to analyze.
    path: Utf8PathBuf,

    #[clap(flatten)]
    flags: Flags,
}

#[derive(Parser, Debug)]
#[group(multiple = false)]
struct Flags {
    #[arg(short, long)]
    /// Count number of characters.
    characters: bool,

    #[arg(short, long)]
    /// Count number of lines.
    lines: bool,

    #[arg(short, long)]
    /// Count number of words. [Default - If no flag is specified, this flag is selected]
    words: bool,
}
fn count_objects(args: Args) -> Result<usize> {
    let file_content = std::fs::read_to_string(args.path)?;
    let flags = (args.flags.characters, args.flags.lines, args.flags.words);
    let number = match flags {
        (true, false, false) => file_content.len(),
        (false, true, false) => file_content.lines().count(),
        (false, false, true | false) => file_content.split_whitespace().count(),
        _ => unreachable!(),
    };

    Ok(number)
}

fn main() {
    let args = Args::parse();

    match count_objects(args) {
        Ok(number) => println!("{number}"),
        Err(err) => eprintln!("{err}"),
    }
}

#[cfg(test)]
mod tests {
    use expect_test::{expect, Expect};

    use super::*;

    fn assert_from_iter<'a>(actual: impl IntoIterator<Item = &'a str>, expected: &Expect) {
        let actual = Args::parse_from(actual);
        expected.assert_eq(&format!("{actual:?}"));
    }

    #[test]
    fn test_flags() {
        let common_args = ["t01", "./test.txt"];
        let args = common_args.into_iter().chain(["-c"]);
        assert_from_iter(
            args,
            &expect![[
                r#"Args { path: "./test.txt", flags: Flags { characters: true, lines: false, words: false } }"#
            ]],
        );

        let args = common_args.into_iter().chain(["-l"]);
        assert_from_iter(
            args,
            &expect![[
                r#"Args { path: "./test.txt", flags: Flags { characters: false, lines: true, words: false } }"#
            ]],
        );

        let args = common_args.into_iter().chain(["-w"]);
        assert_from_iter(
            args,
            &expect![[
                r#"Args { path: "./test.txt", flags: Flags { characters: false, lines: false, words: true } }"#
            ]],
        );

        assert_from_iter(
            common_args,
            &expect![[
                r#"Args { path: "./test.txt", flags: Flags { characters: false, lines: false, words: false } }"#
            ]],
        );
    }
}
