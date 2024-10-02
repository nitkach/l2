fn main() {
    // array of i32 numbers
    let a = [76, 77, 78, 79, 80];

    // we make a slice from the second element to the fifth (excluding)
    let b = &a[1..4];

    // debug print
    println!("{b:?}"); // [77, 78, 79]
}
