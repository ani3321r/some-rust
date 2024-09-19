use raiden_api_crate::PrimaryColors;
use raiden_api_crate::mix; //due to include of reexports we don't have to give reference

fn main(){
    let red = PrimaryColors::Red;
    let yellow = PrimaryColors::Yellow;
    mix(red, yellow);
}

// once we publish a code to crates.io it is permanant, for backward compatibility

// to restrict specific versions from beisng downloaded 
// cargo yank --vers "version number"
// we can undo it be using "--undo"