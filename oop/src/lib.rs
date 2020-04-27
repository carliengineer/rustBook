// //using traits for inheritence 

// pub trait Draw {
//     fn draw(&self);
// }


// //defines a struct named Screen that holds a vector named components. 
// //This vector is of type Box<dyn Draw>, which is a trait object; it’s a stand-in for any type inside a Box that implements the Draw trait.
// pub struct Screen {
//     pub components : Vec<Box<dyn Draw>>,
// }

// impl Screen {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }



// pub struct Button {
//     pub height : u32,
//     pub width : u32,
//     pub label : String,
// }
// // The width, height, and label fields on Button will differ from the fields on other components, 
// // such as a TextField type, that might have those fields plus a placeholder field instead.

// impl Draw for Button {
//     fn draw(&self) {
//         println!("drawing a button!");
//     }
// }