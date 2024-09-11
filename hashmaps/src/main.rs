use std::collections::HashMap;

fn main() {
    let green = String::from("Green");
    let red = String::from("Red");

    let mut scores = HashMap::new(); // initializinng a new hashmap
    // a hashmap takes ownership of the string, i.e., the values are moved

    scores.insert(green, 20);
    scores.insert(red, 30);

    //using get method
    let team_name = String::from("Red");
    let score = scores.get(&team_name); //returns optional value

    for(key, value) in &scores{
        println!("{}: {}", key, value);
    }

    //updating a hashmap
    let mut scores1 = HashMap::new();
    scores1.insert(String::from("blue"), 20);
    scores1.insert(String::from("blue"), 40); // this overwrites the value

    scores1.entry(String::from("yellow")).or_insert(40); // it doesn't exist so new value is inserted
    scores1.entry(String::from("yellow")).or_insert(50); // this will not work as yellow key already exist

    //a little application of hashmap
    let text = "raiden is most powerful with most power";
    let mut map = HashMap::new();
    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}