#[derive(Debug)] // it is a trait
struct Rectangle{
    width: u32,
    height: u32
}

impl Rectangle{
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) ->i32 {
    a + 2
}

// custom fail
pub fn greeting(name: &str) -> String{
    format!("Hello!" )
}

// assert a func to panic
pub struct Guess{
    value: i32,
}

impl Guess{
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("guess must be greater than or equal to 1: {}", value);
        } else if value > 100 {
            panic!("guess must be less than or equal to 100: {}", value);
        }
        Guess{value}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lar_can_hold(){
        let larger = Rectangle{
            width: 10,
            height: 5,
        };
        let smaller = Rectangle{
            width: 8,
            height: 4,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn lar_can_hold_2(){
        let larger = Rectangle{
            width: 10,
            height: 5,
        };
        let smaller = Rectangle{
            width: 8,
            height: 4,
        };

        assert!(!smaller.can_hold(&larger));// assert macro will panic while returning false
    }

    #[test]
    fn it_adds_two(){
        assert_eq!(4, add_two(2)); // checks if equal
    }

    #[test]
    fn not_adds_two(){
        assert_ne!(5, add_two(2)); // checks if not equal
    }

    //custom faliure messages
    #[test]
    fn greeting_contains(){
        let result = greeting("raiden");
        assert!(
            result.contains("raiden"),
            "Greeting do not contain name `{}`",
            result
        );
    }

    // assert a func to panic
    #[test]
    // #[should_panic] // without expected value
    #[should_panic(expected = "guess must be less than or equal to 100")] // due to this the test case passes
    fn greater_than_100(){
        Guess::new(400);
    }

    //test with return type
    #[test]
    fn it_works() -> Result<(), String>{
        if 2+3 == 4{
            Ok(())
        } else{
            Err(String::from("the addition is wrong"))
        }
    }
}