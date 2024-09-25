use anyhow::{anyhow, Result};
use itertools::{repeat_n, Itertools};

pub(crate) fn unpack_string_with_iterators(string: &str) -> Result<String> {
    let unpacked_iterators = string.chars().peekable().batching(|iter| {
        //        ****
        // peek: ^
        // iter: ^
        let char = iter.next()?;

        //        ****
        // peek:  ^
        // iter:  ^
        match char {
            //        N***...
            // peek:  ^
            // iter:  ^
            digit if digit.is_ascii_digit() => {
                Some(Err(anyhow!("found number that doesn't repeating anything")))
            }
            //        \***...
            // peek:  ^
            // iter:  ^
            '\\' => match_escape(iter),
            //        C**
            // peek:  ^
            // iter:  ^
            letter => match_letter(iter, letter),
        }
    });

    unpacked_iterators.process_results(|iter| iter.flatten().collect())
}

fn match_letter(
    iter: &mut std::iter::Peekable<std::str::Chars<'_>>,
    letter: char,
) -> Option<Result<itertools::RepeatN<char>>> {
    //        C**...
    // peek:   ^
    // iter:  ^
    let Some(peeked_char) = iter.peek() else {
        //        C
        // peek:   ^
        // iter:  ^
        return Some(Ok(repeat_n(letter, 1)));
    };

    match peeked_char {
        //        CN*...
        // peek:   ^
        // iter:  ^
        digit if digit.is_ascii_digit() => parse_maybe_number(iter, letter),
        //        CC*...
        // peek:   ^
        // iter:  ^
        //        C\*...
        // peek:   ^
        // iter:  ^
        _ => Some(Ok(repeat_n(letter, 1))),
    }
}

fn parse_maybe_number(
    iter: &mut std::iter::Peekable<std::str::Chars<'_>>,
    repeating_char: char,
) -> Option<Result<itertools::RepeatN<char>>> {
    let Some(char) = iter.peek() else {
        return Some(Ok(repeat_n(repeating_char, 1)));
    };

    if !char.is_ascii_digit() {
        return Some(Ok(repeat_n(repeating_char, 1)));
    }

    let mut number = 0;
    loop {
        let digit = iter
            .next()
            .expect("expected char after peek")
            .to_digit(10)
            .expect("char is digit after check");
        let digit = usize::try_from(digit).expect("this is digit");
        number = number * 10 + digit;

        let Some(char) = iter.peek() else {
            return Some(Ok(repeat_n(repeating_char, number)));
        };

        if !char.is_ascii_digit() {
            return Some(Ok(repeat_n(repeating_char, number)));
        }
    }
}

fn match_escape(
    iter: &mut std::iter::Peekable<std::str::Chars<'_>>,
) -> Option<Result<itertools::RepeatN<char>>> {
    let Some(peeked_char) = iter.peek() else {
        //        \
        // peek:   ^
        // iter:  ^
        return Some(Err(anyhow!(
            "last escape character doesn't escape anything"
        )));
    };

    match peeked_char {
        //        \N**...
        // peek:   ^
        // iter:  ^
        digit_as_char if digit_as_char.is_ascii_digit() => {
            let digit_as_char = iter.next().expect("expected char after peek");
            parse_maybe_number(iter, digit_as_char)
        }
        //        \\**...
        // peek:   ^
        // iter:  ^
        '\\' => {
            let Some('\\') = iter.next() else {
                panic!("expected '\\' after peek");
            };
            //        \\**...
            // peek:   ^
            // iter:   ^
            parse_maybe_number(iter, '\\')
        }
        //        \C**...
        // peek:   ^
        // iter:  ^
        char => Some(Err(anyhow!("cannot escape '{char}'"))),
    }
}

#[cfg(test)]
mod tests {
    use expect_test::{expect, Expect};

    use super::*;

    fn assert_unpack_with_iterators(actual: &str, expected: &Expect) {
        let actual = unpack_string_with_iterators(actual).unwrap();
        expected.assert_eq(&actual);
    }

    #[test]
    fn empty() {
        assert_unpack_with_iterators("", &expect![""]);
    }

    #[test]
    fn unpack_with_iterators() {
        assert_unpack_with_iterators("a4bc2d5e", &expect!["aaaabccddddde"]);
        assert_unpack_with_iterators("abcd", &expect!["abcd"]);
        assert_unpack_with_iterators("aaaaaaaa", &expect!["aaaaaaaa"]);
    }

    #[test]
    fn numbers() {
        assert_unpack_with_iterators("a10", &expect!["aaaaaaaaaa"]);
        assert_unpack_with_iterators("aa11", &expect!["aaaaaaaaaaaa"]);
        assert_unpack_with_iterators("a12a13", &expect!["aaaaaaaaaaaaaaaaaaaaaaaaa"]);
    }

    #[test]
    fn escaping() {
        assert_unpack_with_iterators("qwe\\4\\5", &expect!["qwe45"]);
        assert_unpack_with_iterators("qwe\\45", &expect!["qwe44444"]);
        assert_unpack_with_iterators("qwe\\\\5", &expect![[r"qwe\\\\\"]]);
        assert_unpack_with_iterators("qwe\\\\", &expect![[r"qwe\"]]);
        assert_unpack_with_iterators("qwe\\\\\\\\\\\\\\\\", &expect![[r"qwe\\\\"]]);
    }
}
