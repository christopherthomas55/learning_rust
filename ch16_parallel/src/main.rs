use std::thread;
use std::time::Duration;


fn main() {
    basic_threads();
    msg_parsing();
    shared_state_concurrency();
}


fn basic_threads() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    for i in 1..5 {
        println!("hi number {i} from main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // Without this, main thread will just end without spawned finishing
    handle.join().unwrap();

    // move keyword let's us transfer ownership to thread....borrow checking is niceee
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("hi vector {v:?}");
    });

    handle.join().unwrap();

    println!("\n\n\n\n");
}


use std::sync::mpsc;
fn msg_parsing() {
    // This is the classic go method - send data using messages, not shared memory
    // mpsc stands for multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("you"),
            String::from("are"),
            String::from("a"),
            String::from("loser :P"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    // rx.recv() blocks until msg received, try_recv will soldier on and is good for loops
    // Can also use rx as an iterator
    for received in rx {
        println!("Got: {received}");
    }

    // mpsc can also have multiple producers by cloning transmitter
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("thread a: hi"),
            String::from("thread a: you"),
            String::from("thread a: are"),
            String::from("thread a: a"),
            String::from("thread a: loser :P"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("thread b: hi"),
            String::from("thread b: you"),
            String::from("thread b: are"),
            String::from("thread b: a"),
            String::from("thread b: loser :P"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    for received in rx {
        println!("Got: {received}");
    }
}


use std::sync::{Arc, Mutex};
use std::rc::Rc;
fn shared_state_concurrency() {
    // One of the issues with message parsing is once a msg goes through a channel, you can't use
    // it anymore. Much like single ownership
    //
    // We use classic mutexes for this case where we want multiple users
    // I know what a mutex is ofc, but it stands for mutual explusion. A thread has to acquire the
    // mutex lock to operate on data

    let m = Mutex::new(5);

    {
        // Mutex returns a smartpointer (technically the type MutexGuard)
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");
    println!("Now for the fun");

    // Must use Rc to have multiple owners, but Rc isn't thread safe! It doesn't use concurrency
    // primitives. Arc does
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

    println!("result: {}", *counter.lock().unwrap());

}
