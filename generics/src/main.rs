struct Point<T, U>{
    x: T,
    y: U,
} // we can use generic types in structs too

impl<T, U> Point<T, U>{
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W>{
        Point{
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let largest = get_largest(number_list);

    println!("the larget number is {}", largest);

    let char_list = vec!['a', 'e', 'i', 'o', 'u'];

    let largest = get_largest(char_list);

    println!("the larget char is {}", largest);


    //struct generic types
    let p1 = Point{x:6, y:8};
    let p2 = Point{x:6.5, y:8.5};
    let p3 = Point{x:6, y:8.5}; //in order to use to different types we used two different generics

    //a little example using generic defiantions
    let p1 = Point{x: 6, y: 19.5};
    let p2 = Point{x: "raiden", y: "r"};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
// to return more than one type value in a function we use generics
fn get_largest<T: PartialOrd + Copy>(num_list: Vec<T>) -> T{
    let mut largest = num_list[0];
    for number in num_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
//generics donot reduce performance as during compile time it breaks into the required types and 
//sort which to choose for which 