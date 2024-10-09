mod impl_with_iterators;
mod impl_with_states;

fn main() {
    let string =
        impl_with_iterators::unpack_string_with_iterators("a4bc2d5e").expect("smoke testing");

    println!("{string}");

    let string = impl_with_states::unpack_string_with_state("a4bc2d5e").expect("smoke testing");

    println!("{string}");
}
