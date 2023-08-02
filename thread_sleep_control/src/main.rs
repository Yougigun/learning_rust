use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    // This is our thread which will be put to sleep and woken up later
    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        while !*started {
            started = cvar.wait(started).unwrap();
            println!("Child thread is now running");
        }
    });

    // The main thread simulates some work before waking up the child thread
    println!("Main thread doing some work");
    thread::sleep(std::time::Duration::from_secs(2));

    // After the work is done, we wake up the child thread
    println!("Main thread work finished, waking up child thread");
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    *started = true;
    cvar.notify_one();
}
