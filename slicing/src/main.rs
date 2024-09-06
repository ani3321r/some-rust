fn main() {
    // * slices let us reference a contiguous sequence of elements within a collection, without 
    // referencig the entire collection
    // * slices do not take ownership of the underlying data

    let mut s = String::from("postgress db");
    let word = first_word_len(&s);
    println!("{}",word);
    s.clear();
    // as this way of referencing a part of collection we are gonna use slice now

    let s2 = "next js";
    let word = first_word(s2);
    println!("{}", word);

    //slicing in an array
    let a = [5, 9, 7, 3, 1];
    let slice = &a[0..2];
}


fn first_word_len(s: &String) -> usize{
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item ==b' '{
            return i;
        } 
    }
    s.len()
}


fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}// by using slicing we can return the string