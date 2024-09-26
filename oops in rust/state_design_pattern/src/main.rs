use state_design_pattern::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("raiden is a great warrior");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("raiden is a great warrior", post.content());
}


// main for encoding states
/*
fn main() {
    let mut post = Post::new();

    post.add_text("raiden is a great warrior");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("raiden is a great warrior", post.content());
}
*/
// when not implimenting oops we can include type system and type checking