fn main() {
    // match arms

    #[derive(Debug)]
    enum Language{
        English,
        Hindi,
        Bengali,
        Japanese,
    }

    let language = Language::English;

    match language{
        Language::English => println!("Hello"),
        Language::Hindi => println!("नमस्ते"),
        Language::Bengali => println!("নমস্কার"),
        lang => println!("unsupported language! {:?}", lang),
    }


    // conditional if let
    let auth_status: Option<&str> = None;
    let is_admin = false;
    let grp_id: Result<u8, _> = "36".parse();

    if let Some(status) = auth_status{
        println!("Auth status: {}", status);
    } else if is_admin {
        println!("Auth status: admin");
    } else if let Ok(grp_id) = grp_id {
        if grp_id > 30{
            println!("Auth status: privilaged");
        } else{
            println!("Auth status: basic");
        }
    } else{
        println!("Auth status: guest");
    } 
    // the downside of if let expression is that compiler doesn't enfore it to be exhaustive
    // i.e., even if we remove the else case, there will be no errors(it can introduce defect)


    // while let conditional
    let mut stack = Vec::new();

    stack.push(3);
    stack.push(3);
    stack.push(3);

    while let Some(top) = stack.pop(){
        println!("{}", top);
    }
    // the loop will run as long as the pattern specified will match


    // in for loops
    let v = vec!['a','b','c'];

    for(index, value) in v.iter().enumerate(){
        println!("{} is at index {}", value, index);
    }

    // pattern matching in let statements
    let a = 5;

    // let (a, b) = (4, 5, 6); 
    // here the expression is expecting two integers but we give three
    // so pattern doesn't match

    let (a, b, _) = (4, 5, 6); // to solve the error

}