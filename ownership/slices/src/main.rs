use std::io;


fn main() {
    let mut sentence = String::from("hello world");
    //Having to worry about the index in word getting out of sync with the data in s is tedious and error prone, Rust has a solution to this problem: string slices:
    //let hello = &s[0..5];
    //let world = &s[6..11];

    let word = first_word(&sentence); // we ar egiving immutabl reference
    //slicing like this works with while string we refer to is still there.
    println!("{}", word);
    
    //clera is taking mut reference
    sentence.clear(); // this empties the String, making it equal to ""
    
    //slicing after the org string is gone doesn't work, clear() takes mut reference, and below, we are trying to have immutable reference again.
    //println!("{}", word);
    
    //string literals are slices. That is why they are immutable : &str
    let s = "Hello, world!";
    // it is a slice pointing to that specific point of the binary.



}


//If we have a string slice, we can pass that directly. If we have a String, we can pass a slice of the entire String.
// Defining a function to take a string slice instead of a reference to a String makes our API more general and useful without losing any functionality:

fn first_word(sentence: &str) -> &str {
    let bytes = sentence.as_bytes();
    //iter is a method that returns each element in a collection and that enumerate wraps the result of iter and returns each element as part of a tuple instead
    //The first element of the tuple returned from enumerate is the index, and the second element is a reference to the element.
    // (we get a reference to the element from .iter().enumerate(), we use & in the pattern)
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &sentence[..i];
        }
    }
    &sentence[..]
}
