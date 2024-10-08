//iterator demonstration
/*
pub trait Iterator{
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
*/

#[test]
fn iterator_demonstration(){
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.into_iter();
    //iter->immutable, iter_mut->mutable, into_iter->own types

    assert_eq!(v1_iter.next(), Some(1));
    assert_eq!(v1_iter.next(), Some(2));
    assert_eq!(v1_iter.next(), Some(3));
    assert_eq!(v1_iter.next(), None);
}

// consumer method example for iterators(they basically take iters and return other datatypes)
#[test]
fn iter_sum(){
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}

// closures capturing environment
#[derive(PartialEq, Debug)]
struct Shoe{
    size: u32,
    style: String,
}

fn shoe_same_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe>{
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
} //taking ownership of the vector


// creating own iterator
struct Counter{
    count: u32,
}

impl Counter{
    fn new() -> Counter{
        Counter {count: 0}
    }
}

impl Iterator for Counter{
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item>{
        if self.count < 5{
            self.count += 1;
            Some(self.count)
        } else{
            None
        }
    }
}

#[test]
fn calling_next_directly(){
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

// a more complicated example
#[test]
fn using_other_iter_trait_met() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1)) // take two iterators and zip them up into one
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}

fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter(); // iterators can be applied over any data structure

    for value in v1_iter{
        println!("value: {}", value);
    }

    //adapter methods(takes iters returns iters)
    let v2: Vec<i32> = vec![1, 2, 3];
    let v3: Vec<_> = v2.iter().map(|x| x+1).collect();

    assert_eq!(v3, vec![2, 3, 4]);
}


// testing for closures capturing env
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn filter_by_size(){
        let shoes = vec![
            Shoe{
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe{
                size: 9,
                style: String::from("chappal"),
            },
            Shoe{
                size: 12,
                style: String::from("boot"),
            },
            Shoe{
                size: 10,
                style: String::from("crocs"),
            },
        ];

        let in_my_size = shoe_same_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe{
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe{
                    size: 10,
                    style: String::from("crocs"),
                },
            ]
        );
    }
}