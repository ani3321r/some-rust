pub struct NewsArticle{
    pub author: String,
    pub heaadline: String,
    pub content: String,
}

impl Summary for NewsArticle{
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet{
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String{
        format!("{}, by {}", self.content, self.username)
    }
}

pub trait Summary{
    fn summarize_author(&self) -> String; //it doesn't have a body as we don't want to dictate the implimentation

    fn summarize(&self) -> String{
        format!("(read more from {}...)", self.summarize_author()) //this is the default implimentation
    }
}

//trait bounds
pub fn notify(item: &impl Summary){
    println!("Breaking News!! {}", item.summarize());
}

pub fn notify1<T: Summary>(item1: &T, item2: &T){

}//when we want two items of the same type we use generics for the trait bounds

// for multiple trait bounds and more than one generic we might use "where"
/*
fn some_func<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{

}           
*/

// returning types
fn returns_summarizable() -> impl Summary{
    Tweet{
        username: String::from("Space"),
        content: String::from(
            "a great space exploration story",
        ),
        reply: false,
        retweet: false,
    }
}

// trait bounds for conditional impliment methods
/*
struct Pair<T>{
    x: T,
    y: T
}

impl <T> Pair<T>{
    fn new(x:T, y:T) -> Self{
        Self {x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T>{
    fn cmp_Display(&self){ //this is only available to pair struct
        if self.x >= self.y{
            println!("the largest member is x = {}", self.x);
        } else {
            println!("the largest member is y = {}", self.y);
        }
    }
}
*/


fn main() {
    //implimenting traits
    let tweet = Tweet{
        username: String::from("@raiden12"),
        content: String::from("hello"),
        reply: false,
        retweet: false
    };

    let article = NewsArticle{
        author: String::from("Raiden"),
        heaadline: String::from("a great day"),
        content: String::from("today is a really great day")
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());

    notify(&article);

    //return type exm
    println!("{}", returns_summarizable().summarize());
}
