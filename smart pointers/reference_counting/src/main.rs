use std::rc::Rc;

enum List{
    Cons(i32, Rc<List>), 
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(6,  Rc::new(Cons(2, Rc::new(Cons(4, Rc::new(Nil)))))));
    println!("cnt after creating a = {}", Rc::strong_count(&a));

    let b = Cons(5, Rc::clone(&a));
    println!("cnt after creating b = {}", Rc::strong_count(&a));

    {
        let c = Cons(10, Rc::clone(&a));
        println!("cnt after creating c = {}", Rc::strong_count(&a));
    }
    println!("cnt after c is out of scope = {}", Rc::strong_count(&a));

    // here clone doesn't make deep copies,  it onlly incriments the reference counter 

    // reference counting smart ptrs only allows other parts of the program to read the data
}

// we can infer the reference counting analogy as follows
// everybody enters a room where light is turned on so that everybody can
// see properly but the last one leaving the room must switch off the light


//output
/*
cnt after creating a = 1
cnt after creating b = 2
cnt after creating c = 3
cnt after c is out of scope = 2
*/