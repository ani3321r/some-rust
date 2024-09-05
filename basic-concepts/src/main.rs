fn main() {
    let mut x = 3; // variables are not by default mutable in rust, we have to use the mut for it
    println!("the value is: {}", x);
    x = 5;
    println!("the value of x is: {}", x);

    const RAIDEN: u32 = 5000; //the basic diff b/w const and var is const cannot be mutated using mut
    // consts are also type annotated
    // consts cannot return any value which is computed at runtime, whereas var can
    // const must be uppercase

    let y = 7;
    println!("the value of y is: {}", y);
    let y = "seven"; // when we redeclare the same it is called shadowing
    // now both variables are mutable and types can be changed
    println!("the value of y is: {}", y);


    // diffrent ways of representing integers
    let a = 98_222; // decimal
    let b = 0xff; // hex
    let c = 0o77; // octal
    let d = 0b1111_0000; // binary
    let e = b'A'; // Byte (u8 only)

    let f: u8 = 255; // 256 will become 0 and 257 will become 1(overflow)(only at runtime)

    // floating point numbers
    let k = 2.0;// default is f64
    let g: f32 = 4.2;

    //bools chars and basic sum sub multi div ops are same

    //compound types
    let tup = ("Raiden", 500_000); // automatically annotated
    let (name, score) = tup; // destructuring of a tuple
    let score = tup.1; // dot notation for getting a value from tuple

    let arr = [4, 8, 6];
    let item = arr[2];

    let byte = [0; 8]; // initialize an array with 8 zeros

    let sub = new_func(8, 6);
    println!("the sub is: {}", sub);

    // control flow is same as other languages, if else and else if

    // loops
    let mut cnt = 0;
    let res = loop{
        cnt += 1;

        if cnt == 10{
            break cnt;
        }
    };
    println!("the result is {}", res);

    let mut num = 4;
    while num != 0{
        println!("{}!", num);
        num -= 1;
    }

    let a = [15, 25, 55, 85, 95];

    for element in a.iter(){
        println!("the value is: {}", element);
    } // iterate over the values in a

    for number in 1..4{
        println!("{}!", number);
    } // print numbers from 1 to 3
}

//function name must be all lower case
fn new_func(x: i32, y: i32) -> i32{
    println!("Raiden");
    println!("the sum is {}", x + y);
    x - y // as we are initializing the return type we can just write the operation without anything
}