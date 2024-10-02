fn main() {
    /*
    str representation {
        pointer_to_bytes,
        len,
    }

    & is required for some indirection, because `str` is ?Sized (unsized) type.
     */
    let s1: &str = "hello";

    /*
    `String` is implemented via `Vec`, but can be represented as:
    String representation {
        pointer_to_bytes,
        len,
        capacity,
    }
     */
    let s2 = String::from("hello");

    // simply take the pointer and the length to create `str`
    let s3 = s2.as_str();

    //  5: 5 bytes of UTF-8 encoded chars
    let size_of_s1 = std::mem::size_of_val(s1);

    // 24: 8 bytes (pointer) + 8 bytes (len, usize = pointer size, but depends on machine)
    // + 8 bytes (capacity, usize)
    let size_of_s2 = std::mem::size_of_val(&s2);

    // 16: 8 (pointer) + 8 bytes (len, usize)
    let size_of_s3 = std::mem::size_of_val(&s3);

    println!("{:?}", size_of_s1); // 5

    println!("{:?}", size_of_s2); // 24

    println!("{:?}", size_of_s3); // 16
}
