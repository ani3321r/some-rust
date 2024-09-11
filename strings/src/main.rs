use unicode_segmentation::UnicodeSegmentation; //for getting grapheme cluster we need to include this
fn main() {
    // ways to initialize a string
    let s1 = String::new();
    let s2 = "raiden";
    let s3 = s2.to_string();
    let s4 = String::from("raiden2");

    // as strings are utf-8 encoded many languages are supported
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("😊"); // also has emoji support

    //appending
    let mut s = String::from("postgress");
    s.push_str(" db");
    s.push('!');

    let s1 = String::from("hello ");
    let s2 = String::from("raiden");
    // let s3: String = s1 + &s2;
    let s4 = format!("{}{}",s1,s2); //using format macro

    // a char can't be specified from a string just by using sqare braces and index val, because its 
    // utf-8 encoded
    let hello: String = String::from("नमस्ते");
    // in bytes
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]

    for b in "नमस्ते".bytes(){
        println!("{}", b);
    } //iterates over the bytes of every char

    //scaler values
    // ['न', 'म', 'स', '्', 'त', 'े']

    for c in "नमस्ते".chars(){
        println!("{}", c);
    }

    // Grapheme Clusters
    // ["न", "म", "स्", "ते"]

    for g in "नमस्ते".graphemes(true){
        println!("{}", g);
    }
}