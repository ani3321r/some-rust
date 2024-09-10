fn main() {
    // ways to initialize a string
    let s1 = String::new();
    let s2 = "raiden";
    let s3 = s2.to_string();
    let s4 = String::from("raiden2");

    // as strings are utf-8 encoded many languages are supported
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");

    //appending
    let mut s = String::from("postgress");
    s.push_str(" db");
    s.push('!');

    let s1 = String::from("hello ");
    let s2 = String::from("raiden");
    // let s3: String = s1 + &s2;
    let s4 = format!("{}{}",s1,s2); //using format macro
}