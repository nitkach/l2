use crate::months::Months;
use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use std::collections::HashMap;

/// Sorting options
#[allow(clippy::struct_excessive_bools)]
#[derive(bon::Builder)]
pub(crate) struct Sort {
    /// Column number to sort by
    pub(crate) sort_column: usize,

    /// Sort numbers
    pub(crate) by_numbers: bool,

    /// Sort by numeric value taking into account suffixes
    pub(crate) by_numbers_with_suffixes: bool,

    /// Sort by month
    pub(crate) by_month: bool,

    /// Sort in reverse order
    pub(crate) reverse: bool,

    /// Do not output dublicates
    pub(crate) unique: bool,

    /// Ignore trailing spaces
    pub(crate) ignore_trailing_spaces: bool,

    /// Check if the data is already sorted
    pub(crate) check_sorted: bool,

    pub(crate) separator: String,
}

impl Sort {
    pub(crate) fn sort_contents<'line>(&self, contents: &'line str) -> Result<Vec<&'line str>> {
        let mut sortable = contents.lines().collect::<Vec<_>>();
        let mut message = String::new();

        if self.unique {
            let mut set = std::collections::HashSet::new();
            sortable.retain(|line| set.insert(line.to_owned()));
            message.push_str(", ignoring duplicate rows");
        }

        let sorted = match (
            self.by_numbers,
            self.by_numbers_with_suffixes,
            self.by_month,
        ) {
            (true, false, false) => {
                message = format!(
                    "Sorted by numbers from column {}{message}.",
                    self.sort_column
                );
                self.sort_by_numbers(sortable)
            }
            (false, true, false) => {
                message = format!(
                    "Sorted by numbers with suffixes from column {}{message}.",
                    self.sort_column
                );
                self.sort_by_numbers_with_suffixes(sortable)
            }
            (false, false, true) => {
                message = format!(
                    "Sorted by months from column {}{message}.",
                    self.sort_column
                );
                self.sort_by_months(sortable)
            }
            (false, false, false) => {
                message = format!(
                    "Sorted by strings from column {}{message}.",
                    self.sort_column
                );
                self.sort_by_str(sortable)
            }
            // unreacheble, because there is `multiple = false` above the `SortFlags` structure
            _ => unreachable!(),
        }?;

        println!("{message}");

        Ok(sorted)
    }

    pub(crate) fn check_is_sorted(&self, contents: &str) -> Result<Option<bool>> {
        if !self.check_sorted {
            return Ok(None);
        }

        let mut sortable = contents.lines().collect::<Vec<_>>();

        if self.unique {
            let mut set = std::collections::HashSet::new();
            sortable.retain(|line| set.insert(line.to_owned()));
        }

        let is_sorted = match (
            self.by_numbers,
            self.by_numbers_with_suffixes,
            self.by_month,
        ) {
            (true, false, false) => {
                let comparable_and_line = self.prepare_comparable_numbers_column(sortable)?;
                comparable_and_line
                    .into_iter()
                    .tuple_windows()
                    .all(|((a_cmp, _), (b_cmp, _))| a_cmp <= b_cmp)
            }
            (false, true, false) => {
                let comparable_and_line = self.prepare_comparable_suffixes_column(sortable)?;
                comparable_and_line
                    .into_iter()
                    .tuple_windows()
                    .all(|((a_cmp, _), (b_cmp, _))| a_cmp <= b_cmp)
            }
            (false, false, true) => {
                let comparable_and_line = self.prepare_comparable_months_column(sortable)?;
                comparable_and_line
                    .into_iter()
                    .tuple_windows()
                    .all(|((a_cmp, _), (b_cmp, _))| a_cmp <= b_cmp)
            }
            (false, false, false) => {
                let comparable_and_line = self.prepare_comparable_strs_column(sortable)?;
                comparable_and_line
                    .into_iter()
                    .tuple_windows()
                    .all(|((a_cmp, _), (b_cmp, _))| a_cmp <= b_cmp)
            }
            _ => unreachable!(),
        };

        Ok(Some(is_sorted))
    }

    pub(crate) fn sort<Comparable: Ord, Line>(
        &self,
        mut sortable: Vec<(Comparable, Line)>,
    ) -> Vec<Line> {
        sortable.sort_unstable_by(|(a, _), (b, _)| {
            let cmp = a.cmp(b);

            if self.reverse {
                cmp.reverse()
            } else {
                cmp
            }
        });

        sortable
            .into_iter()
            .map(|(_, line)| line)
            .collect::<Vec<_>>()
    }

    // Sorts lines by the column with numbers when the `-n` flag is specified.
    pub(crate) fn sort_by_numbers<'line>(
        &self,
        sortable: Vec<&'line str>,
    ) -> Result<Vec<&'line str>> {
        let sortable = self.prepare_comparable_numbers_column(sortable)?;

        let sorted = self.sort(sortable);

        Ok(sorted)
    }

    // Sorts lines by the column with numbers with suffixes when `-s` flag is specified.
    pub(crate) fn sort_by_numbers_with_suffixes<'line>(
        &self,
        sortable: Vec<&'line str>,
    ) -> Result<Vec<&'line str>> {
        let sortable = self.prepare_comparable_suffixes_column(sortable)?;

        let sorted = self.sort(sortable);

        Ok(sorted)
    }

    // Sorts lines by the column with months when `-M` flag is specified.
    pub(crate) fn sort_by_months<'line>(
        &self,
        sortable: Vec<&'line str>,
    ) -> Result<Vec<&'line str>> {
        let sortable = self.prepare_comparable_months_column(sortable)?;

        let sorted = self.sort(sortable);

        Ok(sorted)
    }

    // Sorts lines by the column with strings when no sort flag is specified.
    pub(crate) fn sort_by_str<'line>(&self, sortable: Vec<&'line str>) -> Result<Vec<&'line str>> {
        let sortable = self.prepare_comparable_strs_column(sortable)?;

        let sorted = self.sort(sortable);

        Ok(sorted)
    }

    /// Prepares lines by extracting the value from them by column number
    ///
    /// ### Example
    ///
    /// Args:
    ///      - column with numbers: `2`
    ///     - flag to sort numbers: `-n`
    ///
    /// Sort lines:
    /// ```
    ///   apple 2
    /// vanille 1
    /// ```
    /// Input:
    /// ```
    /// ["  apple 2", "vanille 1"]
    /// ```
    ///
    /// Output :
    /// ```
    /// [(2_i64, "  apple 2"), (1_i64, "vanille 1")].
    /// ```
    pub(crate) fn prepare_comparable_numbers_column<'line>(
        &self,
        sortable: Vec<&'line str>,
    ) -> Result<Vec<(i64, &'line str)>> {
        let lines = sortable
            .into_iter()
            .map(|line| {
                let (nth, line) = self.extract_nth_column_from_line(line)?;

                let comparable_number = nth.trim().parse::<i64>().with_context(|| {
                    format!(
                        "Column {} doesn't contain only numbers: \"{nth}\"",
                        self.sort_column
                    )
                })?;

                Ok((comparable_number, line))
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(lines)
    }

    /// Prepares lines by extracting the value from them by column number
    ///
    /// ### Example
    ///
    /// Args:
    ///      - column with numbers: `2`
    ///     - flag to sort numbers: `-s`
    ///
    /// Sort lines:
    /// ```
    ///   apple 2
    /// vanille 1
    /// ```
    /// Input:
    /// ```
    /// ["  apple 2k", "vanille -1M"]
    /// ```
    ///
    /// Output :
    /// ```
    /// [(2000_i64, "  apple 2k"), (-1_000_000_i64, "vanille -1M")].
    /// ```
    pub(crate) fn prepare_comparable_suffixes_column<'line>(
        &self,
        sortable: Vec<&'line str>,
    ) -> Result<Vec<(i64, &'line str)>> {
        let suffixes =
            HashMap::<char, i64>::from_iter([('k', 1_000), ('M', 1_000_000), ('B', 1_000_000_000)]);

        let lines = sortable
            .into_iter()
            .map(|line| self.parse_suffix_number(line, &suffixes))
            .collect::<Result<Vec<_>>>()?;

        Ok(lines)
    }

    /// Prepares lines by extracting the value from them by column number
    ///
    /// ### Example
    ///
    /// Args:
    ///      - column with numbers: `2`
    ///     - flag to sort numbers: `-M`
    ///
    /// Sort lines:
    /// ```
    ///   apple jan
    /// vanille August
    /// ```
    /// Input:
    /// ```
    /// ["  apple jan", "vanille August"]
    /// ```
    ///
    /// Output :
    /// ```
    /// [(Months::January, "  apple jan"), (Months::August, "vanille August")].
    /// ```
    pub(crate) fn prepare_comparable_months_column<'line>(
        &self,
        sortable: Vec<&'line str>,
    ) -> Result<Vec<(Months, &'line str)>> {
        let sortable = sortable
            .into_iter()
            .map(|line| {
                let (str_month, line) = self.extract_nth_column_from_line(line)?;
                let month = Months::try_from(str_month)?;
                Ok((month, line))
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(sortable)
    }

    /// Prepares lines by extracting the value from them by column number
    ///
    /// ### Example
    ///
    /// Args:
    ///      - column with numbers: `1`
    ///
    /// Sort lines:
    /// ```
    ///   apple pie
    /// vanille syrup
    /// ```
    /// Input:
    /// ```
    /// ["  apple pie", "vanille syrup"]
    /// ```
    ///
    /// Output :
    /// ```
    /// [("apple", "  apple pie"), ("vanille", "vanille syrup")].
    /// ```
    pub(crate) fn prepare_comparable_strs_column<'line>(
        &self,
        sortable: Vec<&'line str>,
    ) -> Result<Vec<(&'line str, &'line str)>> {
        let sortable = sortable
            .into_iter()
            .map(|line| self.extract_nth_column_from_line(line))
            .collect::<Result<Vec<_>>>()?;

        Ok(sortable)
    }

    pub(crate) fn extract_nth_column_from_line<'line>(
        &self,
        line: &'line str,
    ) -> Result<(&'line str, &'line str)> {
        let line = self.trim(line);
        let nth = line
            .split(&self.separator)
            .nth(self.sort_column - 1)
            .ok_or_else(|| anyhow!("cannot find specified column: {}", self.sort_column))?;

        Ok((nth, line))
    }

    pub(crate) fn parse_suffix_number<'line>(
        &self,
        line: &'line str,
        suffixes: &HashMap<char, i64>,
    ) -> Result<(i64, &'line str)> {
        let (nth, line) = self.extract_nth_column_from_line(line)?;

        let mut chars = nth.trim().chars().peekable();
        let mut number: i64 = 0;
        let mut sign = 1;

        if let Some('-') = chars.peek() {
            sign = -1;
            chars.next().expect("next chars exists");
        }

        while let Some(char) = chars.peek() {
            if let Some(digit) = char.to_digit(10) {
                number = number * 10 + digit as i64;
                chars.next().expect("next chars exists");
            } else {
                break;
            }
        }

        let suffix = chars.next().ok_or_else(|| {
            anyhow!(
                "suffix not found in column {} in line: {}",
                self.sort_column,
                line
            )
        })?;

        let suffix = suffixes
            .get(&suffix)
            .ok_or_else(|| anyhow!("unknown suffix '{suffix}' in line: {line}"))?;

        number *= suffix * sign;

        Ok((number, line))
    }

    pub(crate) fn trim<'line>(&self, line: &'line str) -> &'line str {
        if self.ignore_trailing_spaces {
            line
        } else {
            line.trim_end()
        }
    }
}

#[cfg(test)]
mod tests {
    use expect_test::{expect, Expect};

    use super::*;

    fn contents() -> &'static str {
        // ordered lines:
        // a, -3, -3k, jan
        // b, -2, -2k, feb
        // c, -1, -1k, march
        // d,  0,  0k, april
        // e,  1,  1k, May
        // f,  2,  2k, Jun
        // g,  3,  3k, july
        // h,  4,  4k, aug
        // i,  5,  5k, sep
        // j,  6,  6k, Oct
        // k,  7,  7M, November
        // l,  8,  8B, jan

        "\
        f,  2,  2k, Jun\n\
        j,  6,  6k, Oct\n\
        h,  4,  4k, aug\n\
        a, -3, -3k, jan\n\
        l,  8,  8B, dec\n\
        g,  3,  3k, july\n\
        c, -1, -1k, march\n\
        i,  5,  5k, sep\n\
        b, -2, -2k, feb\n\
        d,  0,  0k, april\n\
        k,  7,  7M, November\n\
        e,  1,  1k, May\n\
        "
    }

    fn assert_expected(actual: &[&str], expect: &Expect) {
        let actual = actual.join("\n");
        expect.assert_eq(&actual);
    }

    #[test]
    fn test_sort_by_number() {
        let sort = Sort::builder()
            .by_month(false)
            .by_numbers(true)
            .by_numbers_with_suffixes(false)
            .check_sorted(false)
            .ignore_trailing_spaces(false)
            .reverse(false)
            .separator(",".to_owned())
            .sort_column(2)
            .unique(false)
            .build();

        let sorted = sort
            .sort_contents(contents())
            .expect("no Result::Err in tests");

        assert_expected(
            &sorted,
            &expect![[r"
                a, -3, -3k, jan
                b, -2, -2k, feb
                c, -1, -1k, march
                d,  0,  0k, april
                e,  1,  1k, May
                f,  2,  2k, Jun
                g,  3,  3k, july
                h,  4,  4k, aug
                i,  5,  5k, sep
                j,  6,  6k, Oct
                k,  7,  7M, November
                l,  8,  8B, dec"]],
        );
    }

    #[test]
    fn test_sort_by_suffix() {
        let sort = Sort::builder()
            .by_month(false)
            .by_numbers(false)
            .by_numbers_with_suffixes(true)
            .check_sorted(false)
            .ignore_trailing_spaces(false)
            .reverse(false)
            .separator(",".to_owned())
            .sort_column(3)
            .unique(false)
            .build();

        let sorted = sort
            .sort_contents(contents())
            .expect("no Result::Err in tests");

        assert_expected(
            &sorted,
            &expect![[r"
                a, -3, -3k, jan
                b, -2, -2k, feb
                c, -1, -1k, march
                d,  0,  0k, april
                e,  1,  1k, May
                f,  2,  2k, Jun
                g,  3,  3k, july
                h,  4,  4k, aug
                i,  5,  5k, sep
                j,  6,  6k, Oct
                k,  7,  7M, November
                l,  8,  8B, dec"]],
        );
    }

    #[test]
    fn test_sort_by_month() {
        let sort = Sort::builder()
            .by_month(true)
            .by_numbers(false)
            .by_numbers_with_suffixes(false)
            .check_sorted(false)
            .ignore_trailing_spaces(false)
            .reverse(false)
            .separator(",".to_owned())
            .sort_column(4)
            .unique(false)
            .build();

        let sorted = sort
            .sort_contents(contents())
            .expect("no Result::Err in tests");

        assert_expected(
            &sorted,
            &expect![[r"
                a, -3, -3k, jan
                b, -2, -2k, feb
                c, -1, -1k, march
                d,  0,  0k, april
                e,  1,  1k, May
                f,  2,  2k, Jun
                g,  3,  3k, july
                h,  4,  4k, aug
                i,  5,  5k, sep
                j,  6,  6k, Oct
                k,  7,  7M, November
                l,  8,  8B, dec"]],
        );
    }

    #[test]
    fn test_sort_by_str() {
        let sort = Sort::builder()
            .by_month(false)
            .by_numbers(false)
            .by_numbers_with_suffixes(false)
            .check_sorted(false)
            .ignore_trailing_spaces(true)
            .reverse(false)
            .separator(",".to_owned())
            .sort_column(1)
            .unique(false)
            .build();

        let sorted = sort
            .sort_contents(contents())
            .expect("no Result::Err in tests");

        assert_expected(
            &sorted,
            &expect![[r"
                a, -3, -3k, jan
                b, -2, -2k, feb
                c, -1, -1k, march
                d,  0,  0k, april
                e,  1,  1k, May
                f,  2,  2k, Jun
                g,  3,  3k, july
                h,  4,  4k, aug
                i,  5,  5k, sep
                j,  6,  6k, Oct
                k,  7,  7M, November
                l,  8,  8B, dec"]],
        );
    }

    #[test]
    fn test_sort_by_str_with_several_equals_letters() {
        let sort = Sort::builder()
            .by_month(false)
            .by_numbers(false)
            .by_numbers_with_suffixes(false)
            .check_sorted(false)
            .ignore_trailing_spaces(false)
            .reverse(false)
            .separator(",".to_owned())
            .sort_column(2)
            .unique(false)
            .build();

        let contents = "\
        7,ab\n\
        1,aaaab\n\
        6,aac\n\
        8,ac\n\
        5,aab\n\
        2,aaaac\n\
        3,aaab\n\
        4,aaac\n\
        9,ac\n\
        ";

        let sorted = sort
            .sort_contents(contents)
            .expect("no Result::Err in tests");

        assert_expected(
            &sorted,
            &expect![[r"
                1,aaaab
                2,aaaac
                3,aaab
                4,aaac
                5,aab
                6,aac
                7,ab
                8,ac
                9,ac"]],
        );
    }

    #[test]
    fn test_sort_with_unique() {
        let sort = Sort::builder()
            .by_month(false)
            .by_numbers(false)
            .by_numbers_with_suffixes(false)
            .check_sorted(false)
            .ignore_trailing_spaces(false)
            .reverse(false)
            .separator(",".to_owned())
            .sort_column(2)
            .unique(true)
            .build();

        let contents = "\
        0,a\n\
        1,aaaa\n\
        0,a\n\
        4,a\n\
        0,a\n\
        2,aaa\n\
        3,aa\n\
        0,a\n\
        0,a\n\
        ";

        let sorted = sort
            .sort_contents(contents)
            .expect("no Result::Err in tests");

        assert_expected(
            &sorted,
            &expect![[r"
                0,a
                4,a
                3,aa
                2,aaa
                1,aaaa"]],
        );
    }

    #[test]
    fn test_is_sorted_numbers() {
        let sort = Sort::builder()
            .by_month(false)
            .by_numbers(true)
            .by_numbers_with_suffixes(false)
            .check_sorted(true)
            .ignore_trailing_spaces(false)
            .reverse(false)
            .separator(",".to_owned())
            .sort_column(2)
            .unique(false)
            .build();

        let sorted = sort
            .sort_contents(contents())
            .expect("no Result::Err in tests");

        assert_expected(
            &sorted,
            &expect![[r"
                a, -3, -3k, jan
                b, -2, -2k, feb
                c, -1, -1k, march
                d,  0,  0k, april
                e,  1,  1k, May
                f,  2,  2k, Jun
                g,  3,  3k, july
                h,  4,  4k, aug
                i,  5,  5k, sep
                j,  6,  6k, Oct
                k,  7,  7M, November
                l,  8,  8B, dec"]],
        );
    }

    #[test]
    fn test_is_sorted_reversed_months() {
        let sort = Sort::builder()
            .by_month(true)
            .by_numbers(false)
            .by_numbers_with_suffixes(false)
            .check_sorted(true)
            .ignore_trailing_spaces(false)
            .reverse(true)
            .separator(",".to_owned())
            .sort_column(4)
            .unique(false)
            .build();

        let sorted = sort
            .sort_contents(contents())
            .expect("no Result::Err in tests");

        assert_expected(
            &sorted,
            &expect![[r"
                l,  8,  8B, dec
                k,  7,  7M, November
                j,  6,  6k, Oct
                i,  5,  5k, sep
                h,  4,  4k, aug
                g,  3,  3k, july
                f,  2,  2k, Jun
                e,  1,  1k, May
                d,  0,  0k, april
                c, -1, -1k, march
                b, -2, -2k, feb
                a, -3, -3k, jan"]],
        );
    }
}
