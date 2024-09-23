#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}
// here we are combining both refcell and rc in order to get multiple
// owners of mutable data

// rc only stores immutable data, as we are using refcells 
// we can update the data after it is being created   

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10; // automatic dereferencing feature

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}

//output
/*
a after = Cons(RefCell { value: 15 }, Nil)
b after = Cons(RefCell { value: 6 }, Cons(RefCell { value: 15 }, Nil))
c after = Cons(RefCell { value: 10 }, Cons(RefCell { value: 15 }, Nil))
*/
// the update appears in all the three lists


/*
let a = 5;
let b = &mut a; // this is not allowed
*/
// as a is immutable we can't take a muable reference of a


/*
let mut c = 10;
let d = &c;
*d = 15; //this is not allowed
*/
// as d is immutable reference, it can't write over the data