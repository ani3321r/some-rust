fn main() {
    /*
    Ownership Rules
    1. Each value in rust has a variable that's called its owner
    2. there can be only one owner at a time
    3. when the owner goes out of scope, the value will be dropped
     */

    // in case of cpp we have to use new for allocating into heap and 
    // del for deallocating, in rust everything happens automatically
    {// s is not valid as it is not declared yet
        let _s = String::from("raiden"); //s is valid from here, here s is directly allocated in the heap
        // do stuff with s
    }// s is out of scope now, also s is deallocated


    let x = 6;
    let y = x; //copy
    make_copy(y);
    println!("{} this is og", y); // here we can still use y, as integers are by default copied 


    let s1  = String::from("raiden01");
    let s2 = s1; //move (not shallow copy)
    let s3 = s2.clone(); // making a copy
    // println!("{}, hello", s1); //this will not be executed as s1 is moved to s2

    let s4 = take_and_give_back(s3);
    let s5 = gives_ownership();
    println!("s5 = {}, s4= {}", s5, s4);


    let demo_str = String::from("car");
    take_ownership(demo_str); 
    // println!("{}", demo_str); // the variable is moved so demo_str is not usable anymore


    let demo2 = gives_ownership();
    println!("demo2 = {}", demo2);



    //references & borrowing
    let str1 = String::from("postgress");
    let len = calculate_len(&str1);
    println!("the length of '{}' is  {}", str1, len); // as we want to use the str later we are 
    // taking it by reference so that str is not moved


    let mut str2 = String::from("postgress");
    change(&mut str2);


    let mut s = String::from("raiden03");
    let r1 = &mut s;
    // let r2 = &mut s; // here it is not possible as a mutable string can be referenced only once, 
    // it is for safety purposes as it one ptr is reading and another one is modifying, there will be
    // no sync of data
    println!("{}", r1 );


    let mut str3 = String::from("raiden04");
    let r3 = &str3;
    let r5 = &str3;
    let r6 = &str3;
    // let r4 = &mut str3; // this is not possible as after taking a immutable reference we can't
    //  take a mutable reference anymore
    // println!("{}, {}", r4, r3 );

    println!("{}, {}, {}", r5, r3, r6 ); // and it's ok to have many immutable references

    let r7 = &mut str3; // as the immmutable references are out of scope we can take
    // a mutable reference now
    println!("{}", r7);
}

fn take_ownership(some_str: String){
    println!("{}", some_str);
}


fn gives_ownership() -> String{
    let some_str = String::from("raiden02");
    some_str
}


fn take_and_give_back(a_string: String) -> String {
    a_string
}


fn make_copy(some_int: i32){
    println!("{}", some_int)
}


fn calculate_len(s: &String) -> usize{
    let length = s.len();
    length
}// references are immutable by default


// for making it mutable
fn change(some_str: &mut String){
    some_str.push_str(" db");
}


/*
fn dangle() -> String{
    let s = String::from("hello");
    &s
} // rust do not allows anything to do which is memory unsafe
*/