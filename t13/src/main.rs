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
