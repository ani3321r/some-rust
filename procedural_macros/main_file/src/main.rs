use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]

struct Raiden;

fn main() {
   Raiden::hello_macro(); 
}
