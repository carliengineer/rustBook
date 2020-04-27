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

pub mod hosting {
    pub fn add_to_waitlist() {}
}