trait Animal{
    fn speak(&self);
}

#[derive(Debug)]
struct Cat{
    name: String
}

impl Animal for Cat{
    fn speak(&self){
        println!("meow")
    }
}

#[derive(Debug)]
struct Dog{
    name: String
}

impl Animal for Dog{
    fn speak(&self){
        println!("woof");
    }
}


fn main() {
    let lua = "lua".to_owned();
    let elixir = "elixir".to_owned();

    // due to coercion the func is accepting other types
    let raiden = "raiden";
    prnt_animal_name(raiden);

    prnt_animal_name(&lua);

    let cat = Box::new(Cat{name: lua});
    let dog = Box::new(Dog{name: elixir});
    // a reference to a box smart ptr can be coerced to a reference
    let dog2 = Dog{name: raiden.to_owned()};

    print_dog(&dog2);
    print_dog(&dog);

    let animals: Vec<Box<dyn Animal>> = vec![cat, dog];
    // similarly a vector can be coerced to an array
    
    animal_sounds(&animals);
}

// a refernce to a string can be coerced to a string slice
// always prefer coercion target (func will be able to accept more types)
fn prnt_animal_name(name: &str){
    println!("{name}");
}

// to remove a layer of indirection we are just taking refernce to a Dog
fn print_dog(dog: &Dog){
    println!("{:?}", dog);
}

fn animal_sounds(animals: &Vec<Box<dyn Animal>>){
    for a in animals{
        a.speak();
    }
}