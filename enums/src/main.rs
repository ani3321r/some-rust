enum IpAddKind{
    v4(u8,u8,u8,u8),
    v6(String),
}

enum Message{
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}// we could have defined everyting under diff structs but with enum it is grouped together

struct IpAdd{
    kind: IpAddKind,
    address: String,
}

//matching expressions(enum)

#[derive(Debug)]
enum UsState{
    Califonia,
    Alaska,
    Arizona
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quater(UsState),
}

fn value_in_cents(coin: Coin) -> u8{
    match coin{
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quater(state) => {
            println!("State Quater from {:?}!", state);
            25
        },
    }
}

fn main() {
    let four = IpAddKind::v4;
    let six = IpAddKind::v6;

    let localhost = IpAddKind::v4(127, 0, 0, 1);


    //option enum

    // as rust do not have null value we have to use option
    /*
    enum Option<T>{
        Some(T),
        None
    }
    */
    
    let some_number = Some(5); //as we are initializing by a value we don't need to annotate
    let not_num: Option<i32> = None; // for None case as no value is there we have to annotate the type

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; //we can't add a option type to a integer type
    //to make it work
    let sum = x + y.unwrap_or(0);

    //matching enum

    value_in_cents(Coin::Quater(UsState::Alaska));


    // matching and option enum

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}",six);


    // if let statement

    let some_value = Some(3);

    if let Some(3) = some_value{
        println!("three");
    } // here the statement is checked backwards as u can infer from the statement
}

fn route(ip_kind: IpAddKind){}

// matching and option enum together

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x{
        Some(i) => Some(i+1),
        _=> None
    }
}