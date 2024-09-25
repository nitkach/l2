use itertools::{repeat_n, Itertools};

fn unpack_string_with_iterators(string: &str) -> Result<String, String> {
    let unpacked_iterators = string.chars().peekable().batching(|iter| {
        //        ****
        // peek: ^
        // iter: ^
        let char = iter.next()?;
        println!("new iteration: {char}");

        //        ****
        // peek:  ^
        // iter:  ^
        match char {
            //        N***...
            // peek:  ^
            // iter:  ^
            digit if digit.is_ascii_digit() => match_digit(),
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
) -> Option<Result<itertools::RepeatN<char>, String>> {
    //        C**...
    // peek:   ^
    // iter:  ^
    let Some(peeked_char) = dbg!(iter.peek()) else {
        //        C
        // peek:   ^
        // iter:  ^
        return Some(Ok(repeat_n(letter, 1)));
    };

    match peeked_char {
        //        CN*...
        // peek:   ^
        // iter:  ^
        digit if digit.is_ascii_digit() => {
            let digit = dbg!(iter.next().unwrap()).to_digit(10).unwrap();
            //        CN*...
            // peek:   ^
            // iter:   ^
            let mut number = usize::try_from(digit).unwrap();

            loop {
                let Some(char) = dbg!(iter.peek()) else {
                    //        CN
                    // peek:    ^
                    // iter:   ^
                    println!("3) {letter}:{number}");
                    return Some(Ok(repeat_n(letter, number)));
                };

                match char {
                    //        CNN...
                    // peek:    ^
                    // iter:   ^
                    digit if digit.is_ascii_digit() => {
                        let digit = dbg!(iter.next().unwrap()).to_digit(10).unwrap();
                        //        CNN...
                        // peek:    ^
                        // iter:    ^
                        let digit = usize::try_from(digit).unwrap();
                        number = number * 10 + digit;
                    }
                    _ => {
                        //        CNC...
                        // peek:    ^
                        // iter:   ^
                        //        CN\...
                        // peek:    ^
                        // iter:   ^
                        println!("4) {letter}:{number}");
                        return Some(Ok(repeat_n(letter, number)));
                    }
                }
            }
        }
        //        CC*...
        // peek:   ^
        // iter:  ^
        //        C\*...
        // peek:   ^
        // iter:  ^
        _ => {
            println!("5) {letter}:1");
            Some(Ok(repeat_n(letter, 1)))
        }
    }
}

fn match_escape(
    iter: &mut std::iter::Peekable<std::str::Chars<'_>>,
) -> Option<Result<itertools::RepeatN<char>, String>> {
    let Some(peeked_char) = dbg!(iter.peek()) else {
        //        \
        // peek:   ^
        // iter:  ^
        // TODO err
        todo!()
    };

    match peeked_char {
        //        \N**...
        // peek:   ^
        // iter:  ^
        digit_as_char if digit_as_char.is_ascii_digit() => {
            let Some(digit_as_char) = dbg!(iter.next()) else {
                //        \
                // peek:   ^
                // iter:  ^
                // err: number expected
                todo!()
            };
            let Some(char) = iter.peek() else {
                //        \N
                // peek:    ^
                // iter:   ^
                return Some(Ok(repeat_n(digit_as_char, 1)));
            };
            //        \N*
            // peek:    ^
            // iter:   ^
            if char.is_ascii_digit() {
                let char = iter.next().expect("next char exists");
                let digit = char.to_digit(10).expect("char is digit");
                //        \NN...
                // peek:    ^
                // iter:    ^
                let mut number = usize::try_from(digit).unwrap();
                loop {
                    let Some(char) = iter.peek() else {
                        //        \\
                        // peek:    ^
                        // iter:   ^
                        // OR
                        //        \\N...N
                        // peek:         ^
                        // iter:        ^

                        break;
                    };
                    //        \\C*...
                    // peek:    ^
                    // iter:   ^
                    if char.is_ascii_digit() {
                        let char = iter.next().expect("next char exists");
                        //        \\N*...
                        // peek:    ^
                        // iter:    ^
                        let digit = char.to_digit(10).expect("char is digit");
                        let digit = usize::try_from(digit).unwrap();
                        number = number * 10 + digit;
                    } else {
                        //        \\C
                        // peek:    ^
                        // iter:   ^
                        break;
                    }
                }
                return Some(Ok(repeat_n(digit_as_char, number)));
            }
            //        \NC
            // peek:    ^
            // iter:   ^
            println!("0) {digit_as_char}:1");
            Some(Ok(repeat_n(digit_as_char, 1)))
        }
        //        \\**...
        // peek:   ^
        // iter:  ^
        '\\' => {
            let Some('\\') = iter.next() else {
                //        \
                // peek:   ^
                // iter:   ^
                // TODO err: \ expected
                todo!()
            };
            //        \\**...
            // peek:   ^
            // iter:   ^
            let mut number = None::<usize>;
            loop {
                let Some(char) = iter.peek() else {
                    //        \\
                    // peek:    ^
                    // iter:   ^
                    // OR
                    //        \\N...N
                    // peek:         ^
                    // iter:        ^
                    break;
                };
                //        \\C*...
                // peek:    ^
                // iter:   ^
                if char.is_ascii_digit() {
                    let char = iter.next().expect("next char exists");
                    //        \\N*...
                    // peek:    ^
                    // iter:    ^
                    let digit = char.to_digit(10).expect("char is digit");
                    let digit = usize::try_from(digit).unwrap();
                    number = Some(number.map_or(digit, |number| number * 10 + digit));
                } else {
                    //        \\C
                    // peek:    ^
                    // iter:   ^
                    break;
                }
            }
            let Some(number) = number else {
                println!("1) \\:1");
                return Some(Ok(repeat_n('\\', 1)));
            };
            println!("2) \\:{number}");
            Some(Ok(repeat_n('\\', number)))
        }
        //        \C**...
        // peek:   ^
        // iter:  ^
        _ => {
            // TODO err
            todo!()
        }
    }
}

fn match_digit() -> ! {
    // TODO err: expected letter or escape
    todo!()
}

fn main() {
    let string = unpack_string_with_iterators("a4bc2d5e").unwrap();

    println!("{string}");

    // let string = unpack_string_with_iterators("qwe\\4\\5");

    // println!("{string}");

    // let string = unpack_string_with_iterators("qwe\\45");

    // println!("{string}");

    // let string = unpack_string_with_iterators("qwe\\\\5");

    // println!("{string}");
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
        assert_unpack_with_iterators("qwe\\\\5", &expect![[r#"qwe\\\\\"#]]);
    }
}
