/*
We’ve added use adder at the top of the code, which we didn’t need in the unit tests. 
The reason is that each file in the tests directory is a separate crate, so we need to bring our library into each test crate’s scope.
Cargo treats the tests directory specially and compiles files in this directory only when we run cargo test.
*/

/*
use adder;
#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
*/