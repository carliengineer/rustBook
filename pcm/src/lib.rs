//We define a module by starting with the mod keyword and then specify the name of the module (in this case, front_of_house) 
//and place curly brackets around the body of the module. Modules can have modules inside.
// mod front_of_house {
//     // pub keyword is needed to allow that module to be used outside
//     pub mod hosting {
//         // pub keyword is needed to allow that function insude this module to be used outside of this module.
//         pub fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     pub mod serving {
//         fn take_order() {}

//         pub fn serve_order() {}

//         fn take_payment() {}
//     }
// }


//Using a semicolon after mod front_of_house rather than using a block tells Rust to load 
//the contents of the module from another file with the same name as the module.
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant2() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}


fn serve_order2() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order2();
        //crate::front_of_house::serving::serve_order();
        //front_of_house::serving::serve_order();     //doesn't work
    }
    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
    /*
     Also, note that because back_of_house::Breakfast has a private field, the struct needs to provide
     a public associated function that constructs an instance of Breakfast (summer here). 
    */
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

/*
The front_of_house module isn’t public, but because the eat_at_restaurant function is defined in the same module as front_of_house 
(that is, eat_at_restaurant and front_of_house are siblings), we can refer to front_of_house from eat_at_restaurant.
*/


// Use keyword.
//use crate::front_of_house::hosting;


pub fn eat_at_restaurant() {
    //here are 2 ways to call crates:

    // 1-Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // 2-Relative path
    front_of_house::hosting::add_to_waitlist();

    //with use of the Use keyword with the absolute or relative path of hosting
    hosting::add_to_waitlist();

    /*
        Choosing whether to use a relative or absolute path is a decision you’ll make based on your project. 
        The decision should depend on whether you’re more likely to move item definition code separately from or together with the code that uses the item. 
        For example, if we move the front_of_house module and the eat_at_restaurant function into a module named customer_experience,
        we’d need to update the absolute path to add_to_waitlist, but the relative path would still be valid.
        However, if we moved the eat_at_restaurant function separately into a module named dining, 
        the absolute path to the add_to_waitlist call would stay the same, but the relative path would need to be updated. 
        Our preference is to specify absolute paths because it’s more likely to move code definitions and item calls independently of each other.
    */

    // if you want to make an item like a function or struct private, you put it in a module.
    // Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules.



    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    /*
        Because we made the Appetizer enum public, we can use the Soup and Salad variants in eat_at_restaurant. 
        Enums aren’t very useful unless their variants are public; it would be annoying to have to annotate all enum variants with pub in every case, so the default for enum variants is to be public. 
        Structs are often useful without their fields being public, so struct fields follow the general rule of everything being private by default unless annotated with pub.
    */
}