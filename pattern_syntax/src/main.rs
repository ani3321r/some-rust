struct Point{
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

// for nested enums
enum Color{
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message1 {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColor(Color),
}

fn main() {
    // matching against literals
    let x = 10;

    match x {
        4 => println!("four"),
        8 => println!("eight"),
        10 => println!("ten"),
        _ => println!("number not available"),
    }


    // matching named variable
    let x1 = Some(6);
    let y1 = 4;

    match x1{
        Some(20) => println!("got 20"),
        Some(y) => println!("matched, y = {:?}", y), // y is shadowed
        _ => println!("default case, x = {:?}", x), // but x is not
    }


    // matching multiple patterns
    let x2 = 8;

    match x2{
        8 | 10 => println!("eight or ten"),
        4 => println!("four"),
        _ => println!("number not available"),
    }


    // matching range of values
    let x3 = 5;

    match x3{
        1..=5 => println!("one through five"), // inclusive match
        _ => println!("other than 1 to 5"),
    }

    let x4 = 'f'; // we can also use it in characters

    match x4{
        'a'..='j' => println!("early letters"),
        'k'..='z' =>println!("later letters"),
        _ => println!("other letter"),
    }


    // destructing to break values

    // 1.sturcts
    let pnt = Point{x: 2, y: 8};

    let Point{x, y} = pnt;
    assert_eq!(2, x);
    assert_eq!(8, y);

    // 2. named variables and literals
    let pnt1 = Point{x:3, y:6};

    match pnt1{
        Point{x, y: 3} =>{
            println!("on x axis {}", x);
        }
        Point{x: 3, y} =>{
            println!("on y axis {}", y);
        }
        Point{x, y} =>{
            println!("on neither axis ({}, {})", x, y);
        },
    }

    // 3.enums
    let msg = Message::ChangeColor(0, 120, 240);

    match msg{
        Message::Quit => {
            println!("Quit");
        }
        Message::Move { x, y } => {
            println!(
                "Move to x: {} y: {}",
                x, y
            );
        }
        Message::Write(text) => {
            println!("Text Message: {}", text)
        },
        Message::ChangeColor(r,g ,b ) => {
            println!(
                "Change color: red {}, green {}, and blue {}",
                r, g, b 
            )
        }
    }

    // 4. nested enums
    let msg2 = Message1::ChangeColor(Color::Hsv(40, 125, 240));

    match msg2{
        Message1::ChangeColor(Color::Rgb(r, g, b)) => {
            println!(
                "Change color: red {}, green {}, and blue {}",
                r, g, b 
            );
    }

    Message1::ChangeColor(Color::Hsv(h, s, v)) => {
        println!(
            "Change Color: hue {}, saturation {}, value {}",
            h, s, v
        );
    }
    _ => (),
    }

    // 5. complex destructuring (tuple and struct in one)
    let ((feet, inches), Point {x, y}) = 
        ((3, 10), Point {x: 4, y: -4});


    // ignoring values
    foo(5, 7);


    // ignore part of a value
    let mut setting_val = Some(6);
    let new_setting_val = Some(12);

    match(setting_val, new_setting_val){
        (Some(_), Some(_)) => {
            println!("can't overwrite customised val");
        }
        _ => {
            setting_val = new_setting_val;
        }
    }
    println!("setting is {:?}", setting_val);


    // ignoring multiple values
    let nums = (4, 5, 6, 1, 2);

    match nums{
        (first, _, third, _, fifth) => {
            println!("the nums are: {}, {}, {}", first, third, fifth)
        }
    }


    // unused variable
    let _x = 23;


    // range syntax
    let nums1 = (2, 4, 5, 1, 6);

    match nums1{
        (first, .., last) => {
            println!("Some nums: {}, {}", first, last);
        }
    }
}

fn foo(_: i32, y: i32){ // by using "_" we are ignoring the first arg
    println!("only y is used here: {}", y);
}