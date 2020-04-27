use std::io;

fn main() {
    //let s = "hello";
    //String heap
    //str stack -?> immutable
    //println!("{}", s);

    let mut mutable_str = String::from("hello");
    mutable_str.push_str(" world");
    println!("{}", mutable_str);


    let s1 = String::from("hello");
    //let s2 = s1;
    //println!("{}, world!", s1);
    //with only s2 valid, when it goes out of scope, it alone will free the memory, and we’re done.


    let s2 = s1.clone();
    println!("{} world!", s1);


    let a = 5;
    let b = a;
    println!("{}, {}", a, b);
    //this works for "int" because they have a known size at compile time and stored entirely in the stack.
    
    // If a type has the Copy trait, an older variable is still usable after assignment.
    // Rust won’t let us annotate a type with the Copy trait if the type, or any of its parts, has implemented the Drop trait.
    // types that are Copy: any group of simple scalar values can be Copy, and nothing that requires allocation or is some form of resource is Copy.

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

   //println!("{}", s);             //can't do it. value moved into the function.
    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward
   // x = x *2;
   //println!("{}", x);             // can do it, int ownership has copy trait 

   let mut s1 = String::from("hello");
   reference_change(&mut s1);
   println!("{}", s1);


   another_reference_change(&mut s1);
   println!("{}", s1);

   //you can have only one mutable reference to a particular piece of data in a particular scope.SO following fails, if you use both r1 and r2, for example with println!:
  let r1 = &mut s1;
  println!("{}", r1);

  // let r2 = &mut s1;
 // println!("{}, {}", r1, r2);


  /* 
    important point is, if you still define r1 and r2 in the same way, but use at most 1 of them, then compiler doesn't complain
    since the access req has been referenced but has not been actually performed.
    The benefit of having this restriction is that Rust can prevent data races at compile time. A data race is similar to a race condition and happens when these three behaviors occur:
        Two or more pointers access the same data at the same time.
        At least one of the pointers is being used to write to the data.
        There’s no mechanism being used to synchronize access to the data. 
        
    However making scope via {} are ok:
      {
        let r1 = &mut s;

      } // r1 goes out of scope here, so we can make a new reference with no problems.
      let r2 = &mut s;
  */
  
  //We also cannot have a mutable reference while we have an immutable one.Users of an immutable reference don’t expect the values to suddenly change out from under them
  // so following fails because r1 borrwed s1 as mut and r1 is still used in this point:
  let r2 = &s1;
//  println!("{}, {}", r1, r2);

  //but if you don't use r1, meanining that r1's last usage was before decleration of r2, then it works as following: 
  println!("r2 {}", r2);

  //rust has Dangling Pointer protection.
  //  the compiler guarantees that references will never be dangling references:
  // if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does
  // so following doesn't work, with the msg: "this function's return type contains a borrowed value, but there is no value for it to be borrowed from"
  let reference_to_dangling = dangling_pointer();
  println!("{}", reference_to_dangling);

} 

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
}

fn reference_change(some_string: &mut String) {
  some_string.push_str(", yo yo yo");
}

fn another_reference_change(some_string: &mut String) {
  some_string.push_str(", yo yo yo");
}

fn dangling_pointer() -> &String{
  let data = String::from("yo yo yo");
  &data
}