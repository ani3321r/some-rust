use rand; 
// when we include a package in one cargo.toml file the other file
// cannot access it in other package

pub fn add_one(x: i32) -> i32{
    x + 1
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn it_works(){
        assert_eq!(4, add_one(3));
    }
}

// to test only the add-one project we write
// cargo test -p add-one

// when publishing we have to 