use std::ops::Deref;

// defining own smart ptr
struct MyBox<T>(T);

impl<T> MyBox<T>{
    fn new(x: T) -> MyBox<T>{
        MyBox(x)
    }
}
// the mybox is not same as box as we are not storing data in heap in case of mybox

//implimenting deref trait for the ptr
impl<T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &T{
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = &x;
    let z = Box::new(x); // the box is a smart ptr so it will work same as reference
    
    let k = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); 
    // the deref ope will follow the memory address stored in y

    assert_eq!(5, *z); // in the back rust is calling the following code "*(z.deref())" 

    // deref do not return the value, instead it returns reference so that 
    // ownership is not transfered from the smart ptr


    //implicit deref coercion
    let m = MyBox::new(String::from("Raiden"));
    hello(&m);
    // the function require a string but instead we are giving str referce and it still works as
    // &MyBox<String> -> &String -> &str

    // for mutable references we can use DerefMut trait
    // we can't deref a immutable reference to a mutable refernece 
    // but we can deref a mutable referencce to a immutable refernece
}

fn hello(name: &str){
    println!("Hello, {}!", name);
}
