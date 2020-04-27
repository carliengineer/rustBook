use std::thread;
use std::time::Duration;

//Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions
fn simulated_expensive_calculation(intensity: u32) -> u32{
    println!("calculating slowly!");
    thread::wait(Duration::from_secs(2));
    intensity
}


//makes duplicated callls to the simulated_expensive_calculation. 
// To simplify the update when those changes happen, we want to refactor this code so it calls the simulated_expensive_calculation function only once. 
//We also want to cut the place where we’re currently unnecessarily calling the function twice without adding any other calls to that function in the process.
// That is, we don’t want to call it if the result isn’t needed, and we still want to call it only once.
// fn generate_workout(intensity: u32, random_number: u32) {
//     if intensity < 25 {
//         println!(
//             "Today, do {} pushups!",
//             simulated_expensive_calculation(intensity)
//         );
//         println!(
//             "Next, do {} situps!",
//             simulated_expensive_calculation(intensity)
//         );
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!(
//                 "Today, run for {} minutes!",
//                 simulated_expensive_calculation(intensity)
//             );
//         }
//     }
// } 




//Each closure instance has its own unique anonymous type: that is, even if two closures have the same signature, their types are still considered different.
//To define structs, enums, or function parameters that use closures, we use generics and trait bounds
//The Fn traits are provided by the standard library. All closures implement at least one of the traits: Fn, FnMut, or FnOnce. 
//We add types to the Fn trait bound to represent the types of the parameters and return values the closures must have to match this trait bound. 
//In this case, our closure has a parameter of type u32 and returns a u32, so the trait bound we specify is Fn(u32) -> u32.
//We want Cacher to manage the struct fields’ values rather than letting the calling code potentially change the values in these fields directly, so these fields are private.
struct cacher {
    where T: Fn(u32) -> u32
    {
        calculation : T,
        value: Option<u32>,
    }
}
//The Cacher struct has a calculation field of the generic type T. The trait bounds on T specify that it’s a closure by using the Fn trait. 
//The value field is of type Option<u32>. Before we execute the closure, value will be None. When code using a Cacher asks for the result of the closure,
// the Cacher will execute the closure at that time and store the result within a Some variant in the value field. 
//Then if the code asks for the result of the closure again, instead of executing the closure again, the Cacher will return the result held in the Some variant.
//When the calling code needs the result of evaluating the closure, instead of calling the closure directly, it will call the value method. 
//This method checks whether we already have a resulting value in self.value in a Some; if we do, it returns the value within the Some without executing the closure again.

//this cahcer implementation has problems:
//1) The first problem is that a Cacher instance assumes it will always get the same value for the parameter arg to the value method. 
// it will always hold value for first int called it
//2) The second problem with the current Cacher implementation is that it only accepts closures that take one parameter of type u32 and return a u32. 
// We might want to cache the results of closures that take a string slice and return usize values, for example. 
// To fix this issue, introduce more generic parameters to increase the flexibility of the Cacher functionality.

fn generate_workout(intensity: u32, random_number: u32) {
    //we can define a closure and store the closure in a variable rather than storing the result of the function call
    let expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    }
    );

    //Instead of saving the closure in a variable directly, we save a new instance of Cacher that holds the closure. 
    //Then, in each place we want the result, we call the value method on the Cacher instance.
    // We can call the value method as many times as we want, or not call it at all, and the expensive calculation will be run a maximum of once.

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
} 


fn main() {
    let user_intensity_input = 10;
    let random_number = 7;


    generate_workout(
        user_intensity_input,
        random_number
    );

    //closures have an additional capability that functions don’t have: 
    //they can capture their environment and access variables from the scope in which they’re defined.
    let x = 4;
    let equal_to_x = |z| z == x;

    //to force the closure to take ownership of the values it uses in the environment, you can use the move keyword before the parameter list.
    // This technique is mostly useful when passing a closure to a new thread to move the data so it’s owned by the new thread.

    let equal_to_x = move |z| z == x;

}
