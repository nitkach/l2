### Task

Create a Rust function that performs primitive unpacking of a string containing repeating characters.

For example:

    "a4bc2d5e" => "aaaabccddddde"

    "abcd" => "abcd"

    "45" => "" (incorrect string)

    "" => ""

#### Additional

Implement support for escape sequences.

For example:

    qwe\4\5 => qwe45 (*)

    qwe\45 ​​=> qwe44444 (*)

    qwe\\5 => qwe\\\\\ (*)

If an incorrect string was passed, the function should return an error. Write unit tests.
