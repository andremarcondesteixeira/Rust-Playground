use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("From spawned thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("From main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    using_move_closures_with_threads();
    using_channels();
    using_mutex();
}

fn using_move_closures_with_threads() {
    let v = vec![1, 2, 3, 4, 5];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn using_channels() {
    use std::sync::mpsc;

    let (sender, receiver) = mpsc::channel();
    let another_transmitter = mpsc::Sender::clone(&sender);

    thread::spawn(move || {
        let messages = vec![
            String::from("hello"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for message in messages {
            sender.send(message).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let messages = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for message in messages {
            another_transmitter.send(message).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    //let received = receiver.recv().unwrap();
    for message in receiver {
        println!("{}", message);
    }
}

fn using_mutex() {
    use std::sync::{Arc, Mutex};

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
