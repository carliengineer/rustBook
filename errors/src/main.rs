use std::io;
use std::fs;
use std::io::Read;
use std::fs::File;
use std::io::ErrorKind;

/*
    Rust groups errors into two major categories: recoverable and unrecoverable errors. 
    For a recoverable error, such as a file not found error, it’s reasonable to report the problem to the user and retry the operation. 
    Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array.
    Most languages don’t distinguish between these two kinds of errors and handle both in the same way, using mechanisms such as exceptions. Rust doesn’t have exceptions. 
    Instead, it has the type Result<T, E> for recoverable errors and the panic! macro that stops execution when the program encounters an unrecoverable error.
*/

// function to propagate error handling
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) { //last expression, so no need to explicitly say return Ok or Err
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}


// with ? operator for error delegation
fn read_username_from_file_2() -> Result<String, io::Error> {
    // how ? operator works is almost same as match:
        //If the value of the Result is an Ok, the value inside the Ok will get returned from this expression, and the program will continue. 
        //If the value is an Err, the Err will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code.
    let mut f = File::open("hello.txt")?;
    //error values that have the ? operator called on them go through the from function, defined in the From trait in the standard library, which is used to convert errors from one type into another. 
    //When the ? operator calls the from function, the error type received is converted into the error type defined in the return type of the current function
    // This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons.
    //As long as each error type implements the from function to define how to convert itself to the returned error type, the ? operator takes care of the conversion automatically.
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}


// even shorter with chaining function calls:
fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// even shorter:
//Reading a file into a string is a fairly common operation, so Rust provides the convenient fs::read_to_string function that opens the file, creates a new String
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn main() {
    // not recovrable errors with panic:
    //panic!("Hello, world!");

    //A backtrace is a list of all the functions that have been called to get to this point. 
    // RUST_BACKTRACE=1 cargo run


    // Recoverable Errors with Result:
    let f = File::open("hello.txt");
    
    let f = match f {
     // Here we tell Rust that when the result is Ok, return the inner file value out of the Ok variant, and we then assign that file handle value to the variable f
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };


    //same with closures:
    /* //chapter 13 talks about unwrap_or_else -> functional language features
        let f = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });
    */


    //If the Result value is the Ok variant, unwrap will return the value inside the Ok. 
    //If the Result is the Err variant, unwrap will call the panic! macro for us. Here is an example of unwrap in action:
    let f_2 = File::open("hello.txt").unwrap();


    //expect, which is similar to unwrap, lets us also choose the panic! error message. 
    //Using expect instead of unwrap and providing good error messages can convey your intent and make tracking down the source of a panic easier
    let f_3 = File::open("hello.txt").expect("Failed to open hello.txt");


    //A Shortcut for Propagating Errors: the ? Operator
    // we’re only allowed to use the ? operator in a function that returns Result or Option or another type that implements std::ops::Try
    //s
    let read_word = read_username_from_file_3().expect("Couldn't read the word from file");
    println!("{}", read_word);

    // when failure is expected, it’s more appropriate to return a Result than to make a panic! call.



}
