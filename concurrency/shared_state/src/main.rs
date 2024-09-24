use std::sync::{Arc, Mutex}; // as the Rc smart pointer is not thread safe, we use Arc
use std::thread;

fn main() {
    let m = Mutex::new(6);
    // mutex: only one thread can access the data at any given time

    {
        let mut num = m.lock().unwrap();
        //block the current thread and tell lock is about to be aquired

        // we can manipulate the underlying data using deference op
        *num = 10;
    }

    println!("m = {:?}", m);


    // sharing mutex between multiple threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    // Mutex uses interior mutability

    for _ in 0..10{
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }
}
// Muter comes with the risk of Deadlocks