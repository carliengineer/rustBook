#[cfg(test)]




/*
The Rust community thinks about tests in terms of two main categories: unit tests and integration tests. 
Unit tests are small and more focused, testing one module in isolation at a time, and can test private interfaces. 
Integration tests are entirely external to your library and use your code in the same way any other external code would, 
using only the public interface and potentially exercising multiple modules per test.
Writing both kinds of tests is important to ensure that the pieces of your library are doing what you expect them to, separately and together.
*/


/* unit tests 
You’ll put unit tests in the src directory in each file with the code that they’re testing. 
 The convention is to create a module named tests in each file to contain the test functions and to annotate the module with #[cfg(test)].
 The #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when you run cargo test, not when you run cargo build.
*/

fn main() {}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


pub fn greetings(name: &str) -> String {
    format!("Yo {}!", name)
}


pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }


    pub fn new_2(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess {
            value
        }
    }
}

fn prints_and_returns(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

//Because the tests are running at the same time,  make sure your tests don’t depend on each other or on any shared state, 
//including a shared environment, such as the current working directory or environment variables.

mod tests {

    // Note that we’ve added a new line inside the tests module: use super::*;.
    // Because the tests module is an inner module, we need to bring the code under test in the outer module into the scope of the inner module. 
    use super::*;


    //Under the surface, the assert_eq! and assert_ne! macros use the operators == and !=, respectively
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_works2() {
        assert_ne!(2 + 2, 5);
    }

    /*
    #[test]
    fn another() {
        panic!("Make this test fail");
    }*/

    //We give the assert! macro an argument that evaluates to a Boolean. If the value is true, assert! does nothing and the test passes. 
    //If the value is false, the assert! macro calls the panic! macro, which causes the test to fail. 
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
    }
    
    #[test]
    fn test_name(){
       let result = greetings("Gok");
       assert!(result.contains("Gok"), "Naming function did not work, it returned this: {}", result) 
    }

    #[test]
    #[should_panic]
    fn greater_than_100(){
        Guess::new(200);
    }

    //making should_panic more precise with expect
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100_2(){
        Guess::new_2(200);
    }

    //tests that return Result<T, E> when they fail, do not panic
    #[test]
    fn it_works_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    //The output from the test that failed, I got the value 8, appears in the section of the test summary output, which also shows the cause of the test failure.
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns(4);
        assert_eq!(10, value);
    }

    // a few specific tests can be very time-consuming to execute, so you might want to exclude them during most runs of cargo test
    // annotate the time-consuming tests using the ignore attribute to exclude them
    #[test]
    #[ignore]
    fn expensive_test() {
        assert_ne!(3+1, 3);
    }

}
