enum List{
    Cons(i32, Box<List>), //cons list is just like linked lists
    Nil,
}
// we are wraping List with Box as it is infinetely reccursive
// and have no idea how much data to store

use List::{Cons, Nil};


fn main() {
    let b = Box::new(6); //the pointer is stored in the heap
    println!("b = {}", b);
    // when we know how much memory a data will take then only we use pointers


    // reccursive types with boxes
    let list = Cons(1,  Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
