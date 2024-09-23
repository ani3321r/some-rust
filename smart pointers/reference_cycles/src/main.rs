use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node{
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main(){
    // creating a tree
    let leaf = Rc::new(Node{
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // here we are upgrading a weak smart ptr to a Rc smart ptr

    let branch = Rc::new(Node{
        value: 10,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); 
    // borrow the value of the refcell smart ptr to point the branch node

    // as branch is a Rc smart ptr, to make it weak smart ptr
    // we are downgrading 

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // whenever we want to see or mutate a value inside a smart ptr
    // we alwasys upgrade to a Rc smart ptr



    // Strong and weak Count
    let leaf1 = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf1 strong = {}, weak = {}",
        Rc::strong_count(&leaf1),
        Rc::weak_count(&leaf1),
    );

    {
        let branch1 = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf1)]),
        });

        *leaf1.parent.borrow_mut() = Rc::downgrade(&branch1);

        println!(
            "branch1 strong = {}, weak = {}",
            Rc::strong_count(&branch1),
            Rc::weak_count(&branch1),
        );

        println!(
            "leaf1 strong = {}, weak = {}",
            Rc::strong_count(&leaf1),
            Rc::weak_count(&leaf1),
        );
    }

    println!("leaf1 parent = {:?}", leaf1.parent.borrow().upgrade());
    println!(
        "leaf1 strong = {}, weak = {}",
        Rc::strong_count(&leaf1),
        Rc::weak_count(&leaf1),
    );

    //output of strong and weak count
    /*
    leaf1 strong = 1, weak = 0
    branch1 strong = 1, weak = 1
    leaf1 strong = 2, weak = 0
    leaf1 parent = None
    leaf1 strong = 1, weak = 0
    */
}

// we can't go to the branch node from the leaf node as
// leaf node is a child of branch(parent)

// so we are using the weak ptr(parent)