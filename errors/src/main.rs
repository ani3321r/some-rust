use std::fs::{self, File};
use std::io::Read;
use std::io::ErrorKind; //match the type of error we get
use std::io;

fn main() {
    // panic!("crash and burn"); // call an error
    // unrecoverable error
    // example
    a();

    //recoverable error
    enum Result<T, E>{
        Ok(T),
        Err(E),
    } //like option enum, for error handeling result enums are used

    //example
    let f = File::open("hello.txt");
    /*
    let f = match f{
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
     */

    //matching error type
    let f = match f{
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("problem opening file: {:?}", other_error)
            }
        }
    };

    //using closures
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            }) // this is an expression
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    }); // it makes the code shorter and more understandable

    let f1 = File::open("hello.txt").unwrap(); // unwarp works same as match statement in err handeling

    //expect method allows us to pass the message without panic macro
    let f3 = File::open("hello.txt").expect("failed to open hello.txt");

}

//error propagation
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;

    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // }; // instead of using a match expression we can just use a "?"

    let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // } //this match expression is also simplified below

    f.read_to_string(&mut s)?;
    Ok(s) // we can safeky return the string here after checking errors before 
}

// a more simpler way for propagation
/*
fn read_username_from_file() -> Result<String, io::Error>{
    fs::read_to_string("hello.txt")
}
*/

/*
a special type of main function used to return error value of required type

fn main() -> Result<(), Box<dyn Error>>{
    let f = File::open("hello.txt")?;

    Ok(())
}
*/



fn a(){
    b();
}

fn b(){
    c(22);
}

fn c(num: i32){
    if(num == 22){
        panic!("don't pass 22");
    }
}

// we can also create a custom struct for handeling the error