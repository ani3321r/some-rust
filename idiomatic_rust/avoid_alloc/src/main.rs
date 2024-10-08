use std::mem;

#[derive(Debug)]
enum User{
  Reader {name: String},
  Writer {name: String},
  Admin {name: String}
}

fn promote(u: &mut User){
  use User::*;

  *u = match u{
    // take will swap out name for a default val and return the og value
    Reader{name} => Writer{name: mem::take(name)}, //take returns an own type
    Writer{name} => Admin{name: mem::take(name)},
    Admin{name: _ } => return,
  }
}


fn main() {
  let mut user = User::Reader{name: "raiden".to_owned()};
  println!("{user:?}");

  promote(&mut user);
  println!("{user:?}");

  promote(&mut user);
  println!("{user:?}");
}
