struct CusSmartPtr{
    data: String,
}

impl Drop for CusSmartPtr{
    fn drop(&mut self){
        println!("Dropping ptr with data `{}`!", self.data);
    }
}


fn main() {
    let c = CusSmartPtr{
        data: String::from("raiden"),
    };
    let d = CusSmartPtr{
        data: String::from("lui"),
    };

    println!("Smart ptrs Created");

    // c.drop(); // rust doesn't allow us to call the drop method 
    // we are calling free on a memory which is already free in this case
    // because when a var goes out of scope rust calls the drop method automatically
    
    drop(c); // we can do this instead which will have no issue
}


//output
/*
Smart ptrs Created
Dropping ptr with data `lui`!
Dropping ptr with data `raiden`!
*/
// first the ptrs are created and dropped in the order of creation (stack ope)