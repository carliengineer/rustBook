#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

//using traits for inheritence 

pub trait Draw {
    fn draw(&self);
}


//defines a struct named Screen that holds a vector named components. 
//This vector is of type Box<dyn Draw>, which is a trait object; it’s a stand-in for any type inside a Box that implements the Draw trait.
pub struct Screen {
    pub components : Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}



pub struct Button {
    pub height : u32,
    pub width : u32,
    pub label : String,
}
// The width, height, and label fields on Button will differ from the fields on other components, 
// such as a TextField type, that might have those fields plus a placeholder field instead.

impl Draw for Button {
    fn draw(&self) {
        println!("drawing a button!");
    }
}


// Alternatively, defining it with generic type paramter with trait bounds. .
/* 
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
    where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
*/
// A generic type parameter can only be substituted with one concrete type at a time, 
// whereas trait objects allow for multiple concrete types to fill in for the trait object at runtime.
// This restricts us to a Screen instance that has a list of components all of type Button or all of type TextField. 
// If you’ll only ever have homogeneous collections, using generics and trait bounds is preferable 
// because the definitions will be monomorphized at compile time to use the concrete types.
// On the other hand, with the method using trait objects, one Screen instance can hold a Vec<T> that contains a Box<Button> as well as a Box<TextField>.
