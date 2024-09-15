use tests_2;

mod common;

#[test]
fn it_adds_two(){
  common::setup(); // call functions defined on the module
  assert_eq!(4, tests_2::add_two2(2));
}

// for running integration test only
// cargo test --test integration_test