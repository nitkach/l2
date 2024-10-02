fn main() {
    // creating Multi-Producer, Single-Consumer FIFO queue;
    // binding Sender and Receiver to variables
    let (sender, receiver) = std::sync::mpsc::channel::<i32>();

    // spawning a new OS thread and by `move` keyword, we transfer ownership
    // of sender variable to closure
    let handle = std::thread::spawn(move || {
        // loop runs from 0 to 9 (inclusive)
        for i in 0..10 {
            // in each iteration we send the current value `i` through channel
            sender.send(i).unwrap();
        }
    });

    // important!! we must join the handle.
    // `join` blocks current thread until the corresponding thread is done
    // executing. it's important to ensure that there's no race between the threads
    // and main thread
    handle.join().unwrap();

    // receiving values from sender.
    // this `iter()` method will block waiting for messages, and return `None`
    // when channel is closed
    for i in receiver.iter() {
        println!("{i:?}");
    }
}
