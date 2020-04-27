
#![allow(unused_variables)]
use std::env;
use std::process;

use minigrep::Config;

/*
Rust community has developed a process to use as a guideline for splitting the separate concerns of a binary program when main starts getting large. 
The process has the following steps:
    Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
    As long as your command line parsing logic is small, it can remain in main.rs.
    When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.
*/

/*
The responsibilities that remain in the main function after this process should be limited to the following:
    Calling the command line parsing logic with the argument values
    Setting up any other configuration
    Calling a run function in lib.rs
    Handling the error if run returns an error
*/
fn main() {
    let args: Vec<String> = env::args().collect();
  //  println!("{:?}", args);

    //the program’s name takes up the first value in the vector at args[0], so we’re starting at index 1. 
    //unwrap_or_else, which is defined on Result<T, E> by the standard library.
    // Using unwrap_or_else allows us to define some custom, non-panic! error handling. 
    //If the Result is an Ok value, this method’s behavior is similar to unwrap: it returns the inner value Ok is wrapping.
    // However, if the value is an Err value, this method calls the code in the closure,
    // which is an anonymous function we define and pass as an argument to unwrap_or_else.
    //unwrap_or_else will pass the inner value of the Err, which in this case is the static string not enough arguments that we added, 
    //to our closure in the argument err that appears between the vertical pipes
    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });

    //The env::args function returns an iterator! Rather than collecting the iterator values into a vector and then passing a slice to Config::new,
    // now we’re passing ownership of the iterator returned from env::args to Config::new directly.
    let config = Config::new(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    }) 
        
    )
    
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        
        process::exit(1);
    }   

}
