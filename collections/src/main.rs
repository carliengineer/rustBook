


/*
Unlike the built-in array and tuple types, the data these collections point to is stored on the heap,  which means the amount of data does not need to be known at compile time
 and can grow or shrink as the program runs. Three collections that are used very often in Rust programs:
    A vector allows you to store a variable number of values next to each other.
    A string is a collection of characters. We’ve mentioned the String type previously, but in this chapter we’ll talk about it in depth.
    A hash map allows you to associate a value with a particular key. It’s a particular implementation of the more general data structure called a map.
*/

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

#![allow(unused_variables)]
fn main() {
    let mut v_1: Vec<i32> = Vec::new();

    {
        //Rust provides the vec! macro for convenience. The macro will create a new vector that holds the values you give it:
         let v_2 = vec![1,2,3]; 
         //v_2 is defined here
    }
    //v_2 is out of scope
    //v_2.push(2); -> no such variable


    v_1.push(5);
    v_1.push(5);
    v_1.push(5);
    v_1.push(5);
    //v_2 is out of scope

    //accessing vectors' elements
    let third: &i32 = &v_1[2]; //& and [] returns reference. program panics if vector doesn't have this element. Returns none if element doesn't exist
   
    //since mutable borrow occurs for the third, if we try to push something to tht vector again, compiler will give error.
    //this is because vectors might be moved in the memory to some other location upon addition of new elements, so reference to memory location might be changed
   // v_1.push(5);
    
    
    // println!("The third element is {}", third);

    match v_1.get(2) { //.get() returns option
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    println!("Third element is {}", third);

    //iterating over mut arrays elements and changing it
    for i in &mut v_1 {
        // with * operator we are dereferencing
        *i += 50;
    }

    //vector of multiple types of elements, with the usage of enums.
    let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];

}
