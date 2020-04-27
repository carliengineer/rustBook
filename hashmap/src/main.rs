#![allow(unused_variables)]

use std::collections::HashMap;
fn main() {

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    //Another way of constructing a hash map is by using the collect method on a vector of tuples, where each tuple consists of a key and its value.
    //The collect method gathers data into a number of collection types, including HashMap. For example, if we had the team names and initial scores in two separate vectors, 
    //we could use the zip method to create a vector of tuples where “Blue” is paired with 10, and so forth. 
    //Then we could use the collect method to turn that vector of tuples into a hash map:
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();


    //For types that implement the Copy trait, like i32, the values are copied into the hash map. 
    //For owned values like String, the values will be moved and the hash map will be the owner of those value
    let field_name = String::from("Gokhan");
    let field_value = String::from("Blue");

    let mut map_1 = HashMap::new();
    map_1.insert(field_name, field_value);

    // using filed_name gives errror as it is borrowed previously by the map, since string is an "owned" value.
    //println!("{}", field_name);


    let name = map_1.get("Gokhan");
    println!("{:#?}", name);


    //We can iterate over each key/value pair in a hash map in a similar manner as we do with vectors, using a for loop:
    for (key, value) in &map_1{
        println!("{}: {}", key, value);
    }
    // When iterating it does not return option, but the vaule diretly, since it only goes through known key values


    // inserting and overwriting
    scores.insert(String::from("Blue"), 10);

    //Only Inserting a Value If the Key Has No Value: entry method
    //The or_insert method on Entry is defined to return a mutable reference to the value for the corresponding Entry key if that key exists,
    // and if not, inserts the parameter as the new value for this key and returns a mutable reference to the new value.
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    //Updating a Value Based on the Old Value
    //Another common use case for hash maps is to look up a key’s value and then update it based on the old value.

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    /*  ***
        Here we store that mutable reference in the count variable, so in order to assign to that value, we must first dereference count using the asterisk (*). 
        The mutable reference goes out of scope at the end of the for loop, so all of these changes are safe and allowed by the borrowing rules.
    */


}
