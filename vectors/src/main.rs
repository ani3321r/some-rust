fn main() {
    let a = [2, 4, 5];

    //vectors
    let mut v:Vec<i32> = Vec::new(); // initializing a new empty vector
    v.push(3);
    v.push(8); // pushing items into the vector

    let v2 = vec![4, 5, 6, 9]; // initializing a vector using macro, we don't have to specify the type

    let third = &v2[2]; // accessing the third element, taking reference of the vec
    // v2.push(3); // we can't push items as it is a mutable reference and the line before is immutable 
    println!("third element is: {}", third);

    // as vectors are of variable size they are stored in the heap so when we get a runtime error
    // whereas in case of array as it has fixed size we get a compile time error

    //using get method we can handel out of bound error as it is a option type
    match v2.get(7) {
        Some(third) => println!("the third element is {}", third),
        None => println!("No third element"),
    }

    // iterating over vectors
    let mut v3 = vec![1, 2, 3, 4, 5];

    for i in &v3{
        println!("{}", i);
    }

    // taking mutable reference and modifying the value
    for i in &mut v3{
        *i += 30;
    }
    for i in &v3{
        println!("{}", i);
    }

    // stoting enum variables in a vector
    enum SpreadSheetCell{
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(6),
        SpreadSheetCell::Text(String::from("green")),
        SpreadSheetCell::Float(50.12),
    ];

    match &row[1]{
        SpreadSheetCell::Int(i) =>println!("{}", i),
        _=>println!("Not an integer")
    };
}
