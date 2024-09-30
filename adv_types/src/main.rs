/*
impl<T> Option<T>{
    pub fn unwrap(self) -> T{
        match self{
            Some(val) => val,
            None => panic!( // the panic is a never type
                "called `Option::unwrap()` on a `None` value"
            ),
        }
    }

}
*/

fn main() {
    // type aliases
    type Kilometers = i32; // it's just a synonym for a signed 32 bit not a new type 

    let x: i32 = 10;
    let y: Kilometers = 5; // it will be treated just as a signed 32 bit
    
    println!("x + y = {} kilometers", x + y);

    // main use of type alliases is to decrease repetition
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = 
        Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk){
        // some code goes here
    }

    // fn return_long_type() -> Thunk{
    //     // some code goes here
    // }

    
    // reason why never type is required
    /*
    while game_in_progress{
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue, // continue is of never type,
                                // it will move back control to the top of the loop
        };
    }
    */


    // another example of the never type is a loop
    /*
    loop{
        print!("raiden");
    }
    */


    // dynamically sized type, whose size can only be known at runtime
    let s1: &str = "Hello raiden"; //"&str" will store two values
                                   //1. address pointing to the location
                                   //2. length of the string

}

/*
never type
fn bar() -> !{ // the "!" means that the function will never return
 // some code
}
*/


// the sized trait is implicitly added to every generic function
// so generic functions will only work whose size can be known at compile time

// special syntax to bypass it
fn generic<T: ?Sized>(t: &T){
 // some code
}