use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, sleep};
use std::time::Duration;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    // This is our thread which will be put to sleep and woken up later
    let handler = thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        while !*started {
            println!("child thread go to sleep");
            // automatically release lock, and once it wake up, acquiring the lock again.
            started = cvar.wait(started).unwrap();
            println!("Child thread is working. started:{started}");
            thread::sleep(std::time::Duration::from_secs(2));
            println!("Child thread finished");
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
    // Need to drop the lock because when main thread wake up the child thread
    // because cvar will try to acquire the lock again.
    drop(started); 
    let _ = handler.join();
}
