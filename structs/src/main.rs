use std::io;

/*
    Structs let you create custom types that are meaningful for your domain.
    By using structs, you can keep associated pieces of data connected to each other and name each piece to make your code clear. 
    Methods let you specify the behavior that instances of your structs have, 
    and associated functions let you namespace functionality that is particular to your struct without having an instance available.
*/


//Rust has provided a number of traits for us to use with the derive annotation that can add useful behavior to our custom types. 
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


//Structs can have multiple impl blocks, if required.

impl Rectangle{
    //associated function
    fn square(size : u32) -> Rectangle{
        Rectangle{width:size, height:size}
    }

    //methods
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other : &Rectangle) -> bool {
        if (self.width >= other.width && self.height >= other.height) {
            return true
        }
        return false
    }
}

fn main() {
    println!("Hello, world!");
    let rect1 = Rectangle{width: 10, height: 20}; 
    //let area = rect_area(&rect1);
    //println!("{:#?}", rect1);

    println!("{}", rect1.area());


    let rect2 = Rectangle{width:45, height:10};

    println!("{}", rect1.can_hold(&rect2));

    let square_1  = Rectangle::square(10);
    println!("{:#?}", square_1);
}

fn rect_area(rect : &Rectangle) -> u32 {
    rect.width * rect.height
}
