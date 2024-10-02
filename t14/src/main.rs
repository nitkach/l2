fn main() {
    let (tx, rv) = std::sync::mpsc::channel::<i32>();

    let handle = std::thread::spawn(move || {
        for i in 0..10 {
            tx.send(i).unwrap();
        }
    });

    handle.join().unwrap();

    for i in rv.iter() {
        println!("{i:?}");
    }
}
