//Any IP address can be either a version four or a version six address, but not both at the same time. 
//That property of IP addresses makes the enum data structure appropriate, because enum values can only be one of its variants.
#![allow(unused_variables)]
enum IPaddrKind {
    V4,
    V6,
}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }


// associate data to enum, without struct
enum IpAddr {
    V4(String), //V4(u8, u8, u8, u8),
    V6(String),
}



enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

/*
Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent.
This enum is Option<T>, and it is defined by the standard library as follows:
*/
/*
enum Option<T> {
    Some(T),
    None,
}
*/
/*
The Option<T> enum is so useful that it’s even included in the prelude; you don’t need to bring it into scope explicitly. 
In addition, so are its variants: you can use Some and None directly without the Option:: prefix. 
The Option<T> enum is still just a regular enum, and Some(T) and None are still variants of type Option<T>.

<T> means the Some variant of the Option enum can hold one piece of data of any type.
*/




//you have to convert an Option<T> to a T before you can perform T operations with it. 
//Generally, this helps catch one of the most common issues with null: assuming that something isn’t null when it actually is.
// /In order to have a value that can possibly be null, you must explicitly opt in by making the type of that value Option<T>. 


// Match keyword
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!"); // prints this whenever code enters here
            1 //still returns the last value of th eblok, assigns 1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}


// Match in Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

//Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code to be valid.






fn main() {
    let four = IPaddrKind::V4;
    let six = IPaddrKind::V6;

    route(four);
    route(six);

    
// let home = IpAddr {
//     kind: IpAddrKind::V4,
//     address: String::from("127.0.0.1"),
// };

let home2 = IpAddr::V4(String::from("127.0.0.1"));


let some_number = Some(5);
let some_string = Some("a string");
//If we use None rather than Some, we need to tell Rust what type of Option<T> we have,
// because the compiler can’t infer the type that the Some variant will hold by looking only at a None value.
let absent_number: Option<i32> = None;

    // _ placeholder pattern we can use when we don’t want to list all possible values.
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }


    //you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.
    //Choosing between match and if let depends on what you’re doing in your particular situation and whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.
    let some_u8_value2 = Some(0u8);
    if let Some(3) = some_u8_value2 {
        println!("three");
    }

    //else statement with if let
    // let mut count = 0;
    // if let Coin::Quarter(state) = coin {
    //     println!("State quarter from {:?}!", state);
    // } else {
    //     count += 1;
    // }

}


fn route(ip_kind: IPaddrKind) {

}