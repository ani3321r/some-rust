//! # My crate
//! 
//! `my_crate` is a collection of utils to make performing certain
//! calculations more convinient

/// Add one to given number
/// # Example
/// 
/// ```
/// let arg = 6;
/// let ans = my_crate::add_one(arg);
/// 
/// assert_eq!(7, ans);
/// ```

pub fn add_one(x: i32) -> i32{
  x + 1
}



//for documentation purpose we comment using "///"

// to build html docs for out crate we write
// cargo doc --open

// the example is actually a documentation test so we can run cargo test

// to document the item containing the comment we use "//!"