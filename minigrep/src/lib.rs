use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_insensitive: bool,
}

impl Config {
    //&'static str is the type of string literals, which is our error message type for now.

    //The standard library documentation for the env::args function shows that the type of the iterator it returns is std::env::Args.
    //Because we’re taking ownership of args and we’ll be mutating args by iterating over it, we can add the mut keyword into the specification of the args parameter to make it mutable.
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
           // panic!("not enough arguments, should be at least 3");
            return Err("not enough arguments, should be at least 3");
        }
        //We could manage the String data in a number of different ways, but the easiest, though somewhat inefficient, route is to call the clone method on the values. 
        //This will make a full copy of the data for the Config instance to own, which takes more time and memory than storing a reference to the string data. 
        //However, cloning the data also makes our code very straightforward because we don’t have to manage the lifetimes of the references; 
        //in this circumstance, giving up a little performance to gain simplicity is a worthwhile trade-off.
        //let query = args[1].clone();
        //let file_name = args[2].clone();
        //Environment variable
        //let case_insensitive = env::var("CASE_INSENSITIVE").is_err();
               
        //We needed clone here because we have a slice with String elements in the parameter args, now changed with iterators
        //since first element of the args is program name, we next iterator
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        }

        let file_name = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name string"),
        }
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        println!("{}", case_sensitive);
    
        //borrowed string can't be overwritten
        //args[1] = "balbal".to_string();
    
        Ok(Config {query, file_name, case_sensitive})
    }
}

//Box<dyn Error> means the function will return a type that implements the Error trait, but we don’t have to specify what particular type the return value will be. 
//This gives us flexibility to return error values that may be of different types in different error cases. The dyn keyword is short for “dynamic.”
//run function now returns an Ok value in the success case. We’ve declared the run function’s success type as () in the signature,
//which means we need to wrap the unit type value in the Ok value.  This Ok(()) syntax might look a bit strange at first, 
//but using () like this is the idiomatic way to indicate that we’re calling run for its side effects only; it doesn’t return a value we need.
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    /* 
        let contents = fs::read_to_string(config.file_name)
            .expect("Something went wrong while reading the file");
        println!("With text:\n{}", contents);    
    */
    //Instead of allowing the program to panic by calling expect, the run function will return a Result<T, E> when something goes wrong. 
    //This will let us further consolidate into main the logic around handling errors in a user-friendly way.
     
    let contents = fs::read_to_string(config.file_name)?;
    //println!("With text:\n{}", contents);
    
    let result = if config.case_insensitive {
        search_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)  
    };

    

    if result.len() == 0 {
        println!("No text matched for the string {}", config.query);
    } 
    else {
        for line in result {
            println!("{}", line);
        }
    }
    Ok(())
}

//lifetime parameters specify which argument lifetime is connected to the lifetime of the return value.
//we indicate that the returned vector should contain string slices that reference slices of the argument contents (rather than the argument query).
/*
pub fn   search<'a> (query : &str, contents:&'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
   
    //lines() returns an iterator
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
*/

//We can previous code in a more concise way using iterator adaptor methods. Doing so also lets us avoid having a mutable intermediate results vector
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|x| x.contains(query))
        .collect()
}
// Most Rust programmers prefer to use the iterator style. It’s a bit tougher to get the hang of at first, 
// but once you get a feel for the various iterator adaptors and what they do, iterators can be easier to understand. 
// Instead of fiddling with the various bits of looping and building new vectors, the code focuses on the high-level objective of the loop. 
// This abstracts away some of the commonplace code so it’s easier to see the concepts that are unique to this code, 
// such as the filtering condition each element in the iterator must pass.



pub fn search_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));  
    }


    #[test]
    fn case_insensitive(){
        let query = "dUcT";
        let contents = "\
RusT:
Safe, faSt, proDuctive.
Pick tHree.";
        assert_eq!(vec!["Safe, faSt, proDuctive."], search_insensitive(query, contents));
    }
}
