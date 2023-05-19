use std::{
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

fn main() {
    // Using Thread to Run Code Simultaneously
    {
        // Create a new thread with `spawn`
        println!("Create a new thread with `spawn`");
        let handle = thread::spawn(|| {
            for i in 1..=10 {
                println!("hi number {i} from the spawned thread!");
                thread::sleep(Duration::from_millis(1));
            }
        });
        for i in 1..=5 {
            println!("hi number {i} from the main thread!");
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap();
    }

    {
        // Using `move` Closures with Threads because thread may outlive the data it references
        println!("\nUsing `move` Closures with Threads");
        let v = vec![1, 2, 3];
        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });
        // println!("Here's a vector: {v:?}"); // cannot compile because `v` is moved to closure in thread
        handle.join().unwrap();
    }

    {
        // Using `Scoped Threads`
        let mut v = vec![1, 2, 3];
        let mut i = 32;

        // scope will return until all in-scope thread are all jointed, so the lifetime is known
        let scope = thread::scope(|s| {
            s.spawn(|| {
                // Can borrow/move v from the outer scope
                println!("Here's a vector: {:?}", v);
            });
            s.spawn(|| {
                i = 7;
            });
        });

        println!("v = {v:?}");
        println!("i = {}", i);
    }

    {
        // Using Message Passing to Transfer Data Between Threads
        use std::sync::mpsc;
        let (tx, rx) = mpsc::channel(); // this will create a infinite buffer channel
        let tx1 = tx.clone(); // clone one more sender to use in another thread
        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });
        // thread::spawn(move || {
        //     let val = String::from("hi");
        //     tx.send(val).unwrap();
        // });

        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }

    {
        // Sending Multiple Values and Seeing the Receiver Waiting
        println!("\nSending Multiple Values and Seeing the Receiver Waiting");
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                sleep(Duration::from_millis(100));
            }
            drop(tx); // this will close the channel
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }

    // Share-State Concurrency
    {
        println!("\nUsing Mutexes to Allow Access to Data from One Thread at a Time");
        let counter = Arc::new(Mutex::new(0));
        {
            let mut num = counter.lock().unwrap();
            *num += 1;
        }

        println!("Result: {}", *counter.lock().unwrap());
    }

    // Sharing a Mutex<T> Between Multiple Threads
    {
        println!("\nSharing a Mutex<T> Between Multiple Threads");
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
        println!("Result before join:\t{}", *counter.lock().unwrap());
        println!("strong_count before join: {}", Arc::strong_count(&counter));
        for handle in handles {
            handle.join().unwrap();
        }
        println!("Result after join:\t{}", *counter.lock().unwrap());
        println!("strong_count after join: {}", Arc::strong_count(&counter));

    }
}
use std::sync::mpsc;
