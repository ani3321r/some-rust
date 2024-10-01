// function pointers
// passing a func to a func
fn add_one(x: i32) -> i32{
    x + 1
}

// its best practice to write funcs that accepts closures
fn do_twice<T>(f: T, arg: i32) -> i32
where T :  Fn(i32) -> i32{
    f(arg) + f(arg)
} // here we are using a closure trait bound


fn main() {
    let ans = do_twice(add_one, 3);

    println!("ans = {}", ans);


    // another example of using functional pointers
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers
        .iter()
        .map(ToString::to_string) 
        .collect();


    // using tuple structs is basically func calling and getting a return value
    enum Status{
        Value(u32), //it uses parenthesis just like a function
        Stop,
    }
    
    let list_of_stats: Vec<Status> = 
        (0u32..20).map(Status::Value).collect();

}

// returning closures
fn returns_closure() -> impl Fn(i32) -> i32{
    |x| x + 1
}
// this syntax will not work for every cases, like

/*
fn returns_closure(a:i32) -> impl Fn(i32) -> i32{
    if(a > 0){
        move |b| a + b
    } else {
        move |b| a - b
    }

} // this will not work becasue impl trait can only return one type of closure
  // even if the closures seem identical they are of different type
*/

// to fix the issue we are gonna use trait objects
fn returns_closure1(a:i32) -> Box<dyn Fn(i32) -> i32>{
    if(a > 0){
        Box::new(move |b| a + b)
    } else {
        Box::new(move |b| a - b)
    }

}