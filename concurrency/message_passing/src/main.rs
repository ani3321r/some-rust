use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    // usually tx and rx are used as trasnmission and reciver
    // here we are using it to destructure it
    
    thread::spawn(move || {
        let msg = String::from("hi raiden");
        tx.send(msg).unwrap(); //here send takes ownership of the value
    });
    
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // sending multiple values
    let (tx1, rx1) = mpsc::channel();

    let tx2 = tx1.clone(); // a new sending handel

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("one and only"),
            String::from("raiden"),
        ];

        for val in vals{
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // second thread passing value to the main thread
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("from"),
            String::from("raiden"),
        ];

        for val in vals{
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    // when we rerun the program the output can be different depends on the threads

    for received1 in rx1{
        println!("Got: {}", received1);
    }
    // here we don't have to use recv as rx1 is treated as an iterator
    // when the channel closes the iteration will end 
}
