/// What will the program output?
///
/// Program output:
/// 1
/// 4
/// 5
/// 6
/// wrap 8
/// 8
/// 0
/// 3
/// 2
///
///
/// Explain the program output.
///
/// 1      - After creating `Example(1)` we don't binding it anywhere, so the drop is called immediately
/// 4      - In Rust, `_` does not bind values. `_` cannot have ownership of an object. So `Example(4)` is created but not binding to a variable
/// 5      - We assigned ownership of `Some(Example(5))` to _e5, and on the next line _e5 became the owner of `None`. The previous value has no owner, so the value is dropped immediately
/// 6      - Explicitly called drop for e6 variable
/// wrap 8 - Сreated `ExampleWrap(Example(8))` and immediately dropped it, since we do not assign its ownership to the variable
/// 8      - Dropped variable `e` in `<ExampleWrap as Drop>::drop`
/// 0      - drop for `ExampleWrap` field
/// 3      - After exiting `fn main` all variables are dropped in reverse order
/// 2
///
///
/// Explain how Drop works and the order in which they are called.
///
/// In Rust, the drop is called automatically when an object goes out of scope.
/// Or it can be explicitly called via `mem::drop`.
/// Rust automatically calls the destructors of all object's field in order
/// that they declared. Local variables are dropped in reverse order.

struct Example(i32);

impl Drop for Example {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}

struct ExampleWrap(Example);

impl Drop for ExampleWrap {
    fn drop(&mut self) {
        let e = std::mem::replace(&mut self.0, Example(0));
        println!("wrap {}", e.0);
    }
}

fn main() {
    Example(1);

    let _e2 = Example(2);

    let _e3 = Example(3);

    let _ = Example(4);

    let mut _e5;

    _e5 = Some(Example(5));

    _e5 = None;

    let e6 = Example(6);

    drop(e6);

    let e7 = Example(7);

    std::mem::forget(e7);

    ExampleWrap(Example(8));
}
