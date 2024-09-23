use std::{thread, time::Duration};

fn main() {
    // thread::spawn(|| {
    //     for i in 1..10{
    //         println!("num {} from spawned thread", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // }); // when the main thread ends the spawned thread stops executing

    // to finish the spawned thread
    let handle = thread::spawn(|| {
        for i in 1..10{
            println!("num {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap(); // spawned thread executes 1st

    for  i in 1..5{
        println!("num {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    // handle.join().unwrap(); // the main thread then rest of spawned thread


    // move closures with threads
    let v = vec![2, 4, 6];

    let handel1 = thread::spawn(move || {
        println!("vector: {:?}", v);
    });
    // using the move keyword the data is moved to the spawn thread
    // if we don't use it the thread has no idea of the lifetime of v

    // aslo we can't use v after is has been moved

    handel1.join().unwrap();
}