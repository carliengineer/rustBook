// lifetime annotation
// The function signature now tells Rust that for some lifetime 'a, the function takes two parameters, both of which are string slices that live at least as long as lifetime 'a. 
// The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime 'a.
// In practice, it means that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the references passed in.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//It’s possible for structs to hold references, but in that case we would need to add a lifetime annotation on every reference in the struct’s definition.
struct ImportantExcerpt<'a> {
    part: &'a str,
}
//This annotation means an instance of ImportantExcerpt can’t outlive the reference it holds in its part field.


// trait bounds, references, lifetimes all used in one function:
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where T: Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn main() {
    //rust uses borrow checker to determine whether a reference is valid, to protect against dangling pointers
    
    /*
    lifetime annotations
    lifetime annotations describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.
    Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters must start with an apostrophe (') 
    and are usually all lowercase and very short, like generic types. Most people use the name 'a. We place lifetime parameter 
    annotations after the & of a reference, using a space to separate the annotation from the reference’s type.
    */

    //&i32        // a reference
    //&'a i32     // a reference with an explicit lifetime
    //&'a mut i32 // a mutable reference with an explicit lifetime

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);

    /*
    One lifetime annotation by itself doesn’t have much meaning, because the annotations are meant to tell Rust how generic lifetime parameters of multiple references relate to each other. 
    For example, let’s say we have a function with the parameter first that is a reference to an i32 with lifetime 'a. 
    The function also has another parameter named second that is another reference to an i32 that also has the lifetime 'a. 
    The lifetime annotations indicate that the references first and second must both live as long as that generic lifetime.
    */


    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };



    /*The compiler uses three rules to figure out what lifetimes references have when there aren’t explicit annotations:
        The first rule is that each parameter that is a reference gets its own lifetime parameter. In other words, a function with one parameter gets one lifetime parameter:
        fn foo<'a>(x: &'a i32); a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.
        The second rule is if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.
        The third rule is if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters. 
        This third rule makes methods much nicer to read and write because fewer symbols are necessary.
    */

    // 'static reference can live for the entire duration of the program
    let s: &'static str = "I have a static lifetime.";



}
