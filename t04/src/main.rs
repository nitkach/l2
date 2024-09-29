use itertools::Itertools;
use std::collections::HashMap;

fn anagrams(words: &[&str]) -> HashMap<String, Vec<String>> {
    let internal_dictionary = words
        .iter()
        .copied()
        // filter single character words
        // `word.len() > 1` - incorrect, this is length in bytes, not characters
        .filter(|&word| word.chars().count() > 1)
        .map(|word| {
            // example: word = "DbAc"
            let key = word
                .to_lowercase()
                .chars()
                .sorted_unstable()
                .collect::<String>();

            // example: ("abcd", "dbac")
            (key, word.to_lowercase())
        })
        .into_group_map();

    internal_dictionary
        .into_values()
        .map(|value| {
            let (first, rests) = value.split_first().expect("is not empty");
            let mut rest = rests.to_vec();

            // necessary for sets with single word
            rest.push(first.clone());
            let rest = rest
                .into_iter()
                .sorted_unstable()
                .unique()
                .collect::<Vec<_>>();

            (first.clone(), rest)
        })
        .collect()
}

fn main() {
    println!(
        "{:?}",
        anagrams(&["foo", "oof", "bar", "bra", "foo", "baz", "bza", "a", "a", "b", "b",])
    );
}

#[cfg(test)]
mod tests {
    use expect_test::{expect, Expect};

    use super::*;

    fn assert_equal(actual: &[&str], expected: &Expect) {
        let actual = anagrams(actual);
        expected.assert_eq(&format!("{actual:?}"));
    }

    #[test]
    fn test_words() {
        assert_equal(
            &[
                "пятак",
                "пятка",
                "тяпка",
                "листок",
                "слиток",
                "столик",
                "д",
                "п",
                "да",
                "ад",
            ],
            &expect![[
                r#"{"листок": ["листок", "слиток", "столик"], "да": ["ад", "да"], "пятак": ["пятак", "пятка", "тяпка"]}"#
            ]],
        );
    }
}
