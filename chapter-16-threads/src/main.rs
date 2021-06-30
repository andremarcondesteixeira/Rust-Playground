use std::thread;

fn main() {
    use std::time::Duration;

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

    thread::spawn(move || sender.send(String::from("Hello!")).unwrap());

    let received = receiver.recv().unwrap();
    println!("{}", received);
}
