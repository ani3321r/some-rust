use std::fmt::Display;

struct ImportantExcerpt<'a>{
    part: &'a str,
}//lifetime annotation for structs

fn main() {
    //r is a dangling reference as x lifetime ends 
    /*
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {r}");   //----------+
    */

    //here the lifetime of x is still valid so we don't get any error
    let x = 5;     // ----------+-- 'b
                        //           |
    let r = &x;   // --+-- 'a  |
                        //   |       |
    println!("r: {r}"); // --+       |
                        // ----------+


    // all having the same lifetimes
    let str1 = String::from("abc");
    let str2 = String::from("efg");

    let result = longest(str1.as_str(), str2.as_str());
    println!("the longest string is: {}", result);

    // with diff lifetimes
    let str3 = String::from("raiden");
    {
        let str4 = String::from("lui");
        let result = longest(str3.as_str(), str4.as_str());
        println!("the longest string is: {}", result);
    }// this is still valid as str4 lives in the results scope

    //invalid case
    /*
    let str3 = String::from("raiden");
    {
        let str4 = String::from("lui");
        let result = longest(str3.as_str(), str4.as_str());
    }
    println!("the longest string is: {}", result); //str4 do not lives long enough for this result
    */
    //to fix this we can remove the lifetime from y and only return x
    let str3 = String::from("raiden");
    {
        let str4 = String::from("lui");
        let result = longest1(str3.as_str(), str4.as_str());
    }
    println!("the longest string is: {}", result);

    // annotation for structs
    let car = String::from("Porsche 918 spyder");
    let car_company = car.split(" ").next().expect("could not found");
    let i = ImportantExcerpt{
        part: car_company,
    };

    //lifetime elisions
    // 1. Each parameter that is a reference gets its own lifetime parameter
    // 2. If there is exactly one input lifetime parameter, that lifetime is 
    // assigned to all output lifetime parameters
    // 3. If there are multiple input lifetime parameters, but one of them is 
    // &self or &mut self the lifetime of self is assigned to all output lifetime parameters.


    //static lifetime
    let s:&'static str = "this has a static lifetime"; //the reference could live uptil the total duration
    // all string literals have static lifetime cause they are stored in program's binary

    //traits, generics and lifetime all in ome example
    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
     ) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len(){
            x
       } else{
            y
       }
    }    
}                         

// as the function has no idea of the lifetimes of the passed value when we try to return some value
// the borrow checker do not allow to do the action

//generic lifetime annotation(explain lifetimes b/w diff lifetimes)
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len(){
        x
    } else{
        y
    }
}

fn longest1<'a>(x: &'a str, y: &str) -> &'a str{
    x
}// the lifetime of the return value always have to tied to passed value

/*
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
*/
