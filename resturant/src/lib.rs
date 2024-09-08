use rand::{Rng, CryptoRng, ErrorKind::Transient}; //nested paths

//another example
use std::io::{self, Write};

// grouping related code in modules

mod front_of_house1{
    mod hosting{
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving{
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// paths for referring to an item in a module tree

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_resturant1(){
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

// relative paths using super keyword

fn serve_order() {}

mod back_of_house1{
    fn fix_incorrect_order(){
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}
}

// privacy rules for structs

mod back_of_house{
    pub struct Breakfast{
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast{
        pub fn summer(toast: &str) -> Breakfast{
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_resturant(){
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("wheat")
}
//using enums
mod back_of_house2{
    pub enum apetizer{
        Soup,
        Salad,
    }
}// when marking the enum public we are making it's contents public which is very useful

pub fn eat_at_resturant2(){
    let order1 = back_of_house2::apetizer::Soup;
    let order2 = back_of_house2::apetizer::Salad;
}


// the use keyword

mod front_of_house2; //get the contents from a different file
// for modules we need to make file with same name 

// use self::front_of_house2::hosting;
pub use self::front_of_house2::hosting; //using the pub keyword external files can also access it

pub fn eat_at_resturant3(){
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}