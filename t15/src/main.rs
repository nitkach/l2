fn main() {
    let s1 = "hello";

    let s2 = String::from("hello");

    let s3 = s2.as_str();

    let size_of_s1 = std::mem::size_of_val(s1);

    let size_of_s2 = std::mem::size_of_val(&s2);

    let size_of_s3 = std::mem::size_of_val(&s3);

    println!("{:?}", size_of_s1);

    println!("{:?}", size_of_s2);

    println!("{:?}", size_of_s3);
}
