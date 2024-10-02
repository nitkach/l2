// creates a channel, spawns a thread, join handle and waits for all numbers
// from argument numbers to be sent.
// this means that until all the numbers are sent, this function will block execution.
fn as_chan(numbers: &[i32]) -> std::sync::mpsc::Receiver<i32> {
    let (sender, receiver) = std::sync::mpsc::channel();

    let handle = std::thread::spawn({
        let vs = numbers.to_owned();

        move || {
            for v in vs {
                sender.send(v).unwrap();
                std::thread::sleep(std::time::Duration::from_secs(1));
            }

            drop(sender);
        }
    });

    handle.join().unwrap();

    receiver
}

// receives values ​​from two receivers and sends these values ​​to the third channel
fn merge(
    a: std::sync::mpsc::Receiver<i32>,
    b: std::sync::mpsc::Receiver<i32>,
) -> std::sync::mpsc::Receiver<i32> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut a_done = false;

    let mut b_done = false;

    // this loop will be executed sequentially:
    //  first match: received 1 from first (a) channel, send to another channel
    // second match: received 2 from second (b) channel, send to another channel
    //  first match: received 3 from first (a) channel, send to another channel
    // ...
    //  first match: received Err(...) - sender dropped, channel closed.
    // second match: received Err(...) - sender dropped, channel closed.
    // exiting loop
    loop {
        // attempt to receive value from first channel without blocking
        match a.try_recv() {
            Ok(i) => {
                tx.send(i).unwrap();
            }

            Err(_) => {
                a_done = true;
            }
        }

        // attempt to receive value from second channel without blocking
        match b.try_recv() {
            Ok(i) => {
                tx.send(i).unwrap();
            }

            Err(_) => {
                b_done = true;
            }
        }

        if a_done && b_done {
            break;
        }
    }

    rx
}

/*
[sender_a]--->[receiver_a->sender<-receiver_b]<---[sender_b]
                             ||
                             ||
                             \/
                         [receiver]
                   prints received values

              output: [1, 2, 3, 4, 5, 6, 7, 8]

*/
fn main() {
    let a = as_chan(&vec![1, 3, 5, 7]);

    let b = as_chan(&vec![2, 4, 6, 8]);

    let c = merge(a, b);

    for v in c.iter() {
        println!("{v:?}");
    }
}
