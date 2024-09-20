use add_one;

fn main() {
    let num = 5;
    println!(
        "{} plus one is {} ",
        num,
        add_one::add_one(num)
    );
}
// after adding a second package workspace we do the following
// 1. cargo build
// 2. cargo run -p adder(package name)