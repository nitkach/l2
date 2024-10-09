use anyhow::{anyhow, Result};
use itertools::repeat_n;

#[derive(Debug)]
enum State {
    Start,
    Character(char, CharacterRepeat),
    Escape(EscapeEntry),
}

#[derive(Clone, Copy, Debug)]
enum CharacterRepeat {
    One,
    Many(usize),
}

#[derive(Debug)]
enum EscapeEntry {
    First,
    Second,
    Repeat(usize),
}

#[derive(Debug)]
enum CharKind {
    Digit(usize),
    Character(char),
    Escape,
}

fn determine_char_kind(char: char) -> CharKind {
    match char {
        digit if digit.is_ascii_digit() => {
            let digit = digit
                .to_digit(10)
                .expect("char is digit")
                .try_into()
                .expect("u32 is digit");
            CharKind::Digit(digit)
        }
        '\\' => CharKind::Escape,
        character => CharKind::Character(character),
    }
}

pub(crate) fn unpack_string_with_state(string: &str) -> Result<String> {
    let mut result = Vec::new();
    let mut state = State::Start;

    for char in string.chars() {
        let char_kind = determine_char_kind(char);
        match (state, char_kind) {
            (State::Start, CharKind::Digit(_)) => {
                return Err(anyhow!("found digit that doesn't repeating anything"))
            }
            (State::Start, CharKind::Character(character)) => {
                state = State::Character(character, CharacterRepeat::One);
            }
            (State::Start, CharKind::Escape) => state = State::Escape(EscapeEntry::First),
            (State::Character(character, repeat), CharKind::Digit(number)) => match repeat {
                CharacterRepeat::One => {
                    state = State::Character(character, CharacterRepeat::Many(number));
                }
                CharacterRepeat::Many(repeat) => {
                    state =
                        State::Character(character, CharacterRepeat::Many(repeat * 10 + number));
                }
            },
            (State::Character(character, repeat), CharKind::Character(next_character)) => {
                push_repeating_character(repeat, &mut result, character);
                state = State::Character(next_character, CharacterRepeat::One);
            }
            (State::Character(character, repeat), CharKind::Escape) => {
                push_repeating_character(repeat, &mut result, character);
                state = State::Escape(EscapeEntry::First);
            }
            (State::Escape(entry), CharKind::Digit(digit)) => match entry {
                EscapeEntry::First => {
                    state = State::Character(
                        char::from_digit(u32::try_from(digit).expect("digit"), 10).expect("digit"),
                        CharacterRepeat::One,
                    );
                }
                EscapeEntry::Second => state = State::Escape(EscapeEntry::Repeat(digit)),
                EscapeEntry::Repeat(repeat) => {
                    state = State::Escape(EscapeEntry::Repeat(repeat * 10 + digit));
                }
            },
            (State::Escape(entry), CharKind::Character(character)) => {
                match entry {
                    EscapeEntry::First => return Err(anyhow!("cannot escape '{char}'")),
                    EscapeEntry::Second => {
                        result.push(repeat_n('\\', 1));
                    }
                    EscapeEntry::Repeat(repeat) => {
                        result.push(repeat_n('\\', repeat));
                    }
                }
                state = State::Character(character, CharacterRepeat::One);
            }
            (State::Escape(entry), CharKind::Escape) => match entry {
                EscapeEntry::First => state = State::Escape(EscapeEntry::Second),
                EscapeEntry::Second => {
                    result.push(repeat_n('\\', 1));
                    state = State::Escape(EscapeEntry::First);
                }
                EscapeEntry::Repeat(repeat) => {
                    result.push(repeat_n('\\', repeat));
                    state = State::Escape(EscapeEntry::First);
                }
            },
        }
    }

    match state {
        State::Start => {}
        State::Character(character, repeat) => match repeat {
            CharacterRepeat::One => result.push(repeat_n(character, 1)),
            CharacterRepeat::Many(repeat) => result.push(repeat_n(character, repeat)),
        },
        State::Escape(escape_entry) => match escape_entry {
            EscapeEntry::First => {
                return Err(anyhow!("last escape character doesn't escape anything"))
            }
            EscapeEntry::Second => result.push(repeat_n('\\', 1)),
            EscapeEntry::Repeat(repeat) => result.push(repeat_n('\\', repeat)),
        },
    }

    Ok(result.into_iter().flatten().collect())
}

fn push_repeating_character(
    repeat: CharacterRepeat,
    result: &mut Vec<itertools::RepeatN<char>>,
    character: char,
) {
    match repeat {
        CharacterRepeat::One => {
            result.push(repeat_n(character, 1));
        }
        CharacterRepeat::Many(repeat) => {
            result.push(repeat_n(character, repeat));
        }
    }
}

#[cfg(test)]
mod tests {
    use expect_test::{expect, Expect};

    use super::*;

    #[track_caller]
    fn assert_unpack_with_state(actual: &str, expected: &Expect) {
        let actual = unpack_string_with_state(actual).expect("no Result::Err in tests");
        expected.assert_eq(&actual);
    }

    #[test]
    fn empty() {
        assert_unpack_with_state("", &expect![""]);
    }

    #[test]
    fn unpack() {
        assert_unpack_with_state("a4bc2d5e", &expect!["aaaabccddddde"]);
        assert_unpack_with_state("abcd", &expect!["abcd"]);
        assert_unpack_with_state("aaaaaaaa", &expect!["aaaaaaaa"]);
    }

    #[test]
    fn numbers() {
        assert_unpack_with_state("a10", &expect!["aaaaaaaaaa"]);
        assert_unpack_with_state("aa11", &expect!["aaaaaaaaaaaa"]);
        assert_unpack_with_state("a12a13", &expect!["aaaaaaaaaaaaaaaaaaaaaaaaa"]);
    }

    #[test]
    fn escaping() {
        assert_unpack_with_state("qwe\\4\\5", &expect!["qwe45"]);
        assert_unpack_with_state("qwe\\45", &expect!["qwe44444"]);
        assert_unpack_with_state("qwe\\\\5", &expect![[r"qwe\\\\\"]]);
        assert_unpack_with_state("qwe\\\\", &expect![[r"qwe\"]]);
        assert_unpack_with_state("qwe\\\\\\\\\\\\\\\\", &expect![[r"qwe\\\\"]]);
    }
}
