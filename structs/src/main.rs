struct User{
    username: String,
    email: String,
    sign_in_cnt: u64,
    active: bool,
}

#[derive(Debug)] // it is a trait
struct rectangle{
    width: u32,
    height: u32
}

impl rectangle{
    fn area_impl(&self) -> u32{
        self.width * self.height
    }

    fn can_hold(&self, other: &rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> rectangle{
        rectangle{
            width: size,
            height: size
        }
    }
} // implimentation block

fn main() {
    let mut user1 = User{
        email: String::from("raiden@gmail.com"),
        username: String::from("raiden123"),
        active: true,
        sign_in_cnt: 3
    }; // we cant just make one property mutable, we have to mutate the whole thing

    let name = user1.username;
    user1.username = String::from("liu123");

    let user2 = build_user(
        String::from("abc@gmail.com"),
        String::from("abc123")
    );

    let user3 = User{
        email: String::from("efg@gmail.com"),
        username: String::from("efg123"),
        ..user2 // the other fields will come from user2
    };


    // tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // usage of tuple struct
    let rect = (40, 20);
    println!("area of rectangle is {}", area(rect));

    // get area using struct 
    let rect1 = rectangle{
        width:50,
        height:30
    };
    println!("area of next rectangle is {}", area_stru(&rect1));

    println!("area of next rectangle is {}", rect1.area_impl()); // finding area using implimentation block

    println!("rect: {:?}", rect1); // syntax for printing values inside of a struct


    // comparing two rectangles
    let rect2 = rectangle{
        width: 20,
        height: 50
    };

    let rect3 = rectangle{
        width: 10,
        height: 30
    };

    println!("rect2 can hold rect3: {}", rect2.can_hold(&rect3));

    let rect4 = rectangle::square(40);
    println!("rect: {:?}", rect4);
}


fn build_user(email:String, username:String) -> User{
    User{
        email,//short hand syntax
        username,
        active: false,
        sign_in_cnt: 0,
    }
}


fn area(dimensions: (u32, u32)) -> u32{
    dimensions.0 * dimensions.1
}


fn area_stru(Rectangle: &rectangle) -> u32{
    Rectangle.width * Rectangle.height
}