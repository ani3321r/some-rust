use trait_objects::{Draw, Screen, Button};

struct SelectBox{
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox{
    fn draw(&self){
        
    }
}

fn main() {
    let screen = Screen{
        components: vec![
            // Box::new(String::from("test")), //if we don't use the draw trait we can't use it in list
            Box::new(SelectBox{
                width: 100,
                height: 100,
                options: vec![
                    String::from("Yes"),
                    String::from("No"),
                ]
            }),
            Box::new(Button {
                width: 100,
                height: 100,
                label: String::from("ok")
            })
        ],
    };

    screen.run();
}
// when using trait object the compiler uses dynamic dispatch as compiler don't know alll
// the object its gonna use during compile time(runtime performance cost)