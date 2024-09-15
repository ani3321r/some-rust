//showing output
fn prints_and_returns_10(a: i32) -> i32{
    println!("Got the value {}", a );
    10
}

//subset of test
pub fn add_two(a: i32) -> i32{
    a + 2
}

//test organizations
pub fn add_two2(a: i32) -> i32{
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32{
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    //controlling tests
    #[test]
    fn it_works() -> Result<(), String>{
        if 2+2 == 4{
            Ok(())
        } else{
            Err(String::from("the addition is wrong"))
        }
    }

    #[test]
    fn it_works_2(){
        assert_eq!(2+2, 4);
    }
    //when we want to run test using a single thread we can use this command
    //cargo test -- --test-threads=1
    // it is helpful when we have a project which will modify some file on the disk

    #[test]
    fn pass_test(){
        let value = prints_and_returns_10(5);
        assert_eq!(10, value);
    }

    #[test]
    fn fail_test(){
        let value = prints_and_returns_10(12);
        assert_eq!(6, value);
    }
    //to get output for every cases we have the command
    //cargo test -- --show-output


    //subset tests
    #[test]
    fn add_two_and_two(){
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_five_and_two(){
        assert_eq!(7, add_two(5));
    }

    #[test]
    fn one_hundred(){
        assert_eq!(102, add_two(100));
    }
    // to test only specific subset
    // cargo test one_hundred
    // cargo test add (for tests starting with add)


    // ignore a test
    #[test]
    #[ignore]
    fn expensive_test(){
        // code that requires a lot of time to run
    }
    // to run only the ignored test
    // cargo test -- --ignored


    //in rust tests can access private functions
    #[test]
    fn internal(){
        assert_eq!(4, internal_adder(2,2));
    }
}
