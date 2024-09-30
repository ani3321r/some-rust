use std::ops::Add;
use std::fmt;

// associated types
pub trait Iterator{
    type Item;

    fn next(&mut self) -> Option<Self::Item>; // we don't know the type until its implimented
}

// we can't have two implimentations where items have different types in associated types
// which we can do otherwise using generics
/*
struct Counter{}

impl Iterator for Counter{
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item>{
        Some(0)
    }
}

impl Iterator for Counter{
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item>{
        Some(0)
    }
}
*/


// default generic type params and operator overloading
#[derive(Debug, PartialEq)]
struct Point{
    x: i32,
    y: i32,
}

impl Add for Point{
    type Output = Point; // specifing concrete type

    fn add(self, other: Point) -> Point{
        Point{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// implimentation of Add trait
/*
trait Add<Rhs=Self>{
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
*/


// case where we need to specify the type passed to add method
struct Millimeters(u32);

struct Meters(u32);

impl Add<Meters> for Millimeters{
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters{
        Millimeters(self.0 + (other.0*1000))
    }
}
/*
Reasons to use generic type params
1.extend a type without breaking existing code
2.allow customization for specific cases
*/



// calling method with same name
trait Pilot{
    fn fly();
}

trait Wizard{
    fn fly();
}

struct Human;

impl Human{
    fn fly(){
        println!("waving arms");
    }
}

impl Wizard for Human{
    fn fly(){
        println!("go up");
    }
}

impl Pilot for Human{
    fn fly(){
        println!("this is your captain speaking");
    }
}


// supertraits
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point1{
    x: i32,
    y: i32,
}

impl OutlinePrint for Point1{} // as long as the display trait is not available this would give an error

impl fmt::Display for Point1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}



// in order to get around the orphan rule we use the new type pattern

struct Wrapper(Vec<String>); //tuple struct with one field of the type we are wrapping

// diplay trait and vector type are both outside the crate
impl fmt::Display for Wrapper{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    assert_eq!(
        Point{x:2, y:3} + Point{x:3, y:2},
        Point{x:5, y:5}
    );


    // this part is calling methods with same name
    /*
    //when using &self param
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
    */

    // now using associated funcs for calling methods with same name
    Human::fly();
    <Human as Wizard>::fly();


    // new type pattern main
    let w = Wrapper(
        vec![String::from("hello"), String::from("Raiden")]
    );
    println!("w = {}", w);
}