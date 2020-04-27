use std::ops::Deref;
use std::mem::drop;

//Box<T>. Boxes allow you to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data
//Boxes don’t have performance overhead, other than storing their data on the heap instead of on the stack. But they don’t have many extra capabilities either. 
//You’ll use them most often in these situations:
        // When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
        // When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
        // When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type


//A cons list is a data structure that comes from the Lisp programming language and its dialects. In Lisp, the cons function 
//(short for “construct function”) constructs a new pair from its two arguments, which usually are a single value and another pair.
//Each item in a cons list contains two elements: the value of the current item and the next item. 
//The last item in the list contains only a value called Nil without a next item. A cons list is produced by recursively calling the cons function.
        
// enum List {
//     //we cant leave as Cons(i32, List), compiler warns:help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to make `List` representable
//     // “indirection” means that instead of storing a value directly, we’ll change the data structure to store the value indirectly by storing a pointer to the value instead.
//     Cons(i32, List),
//     Nil,
// }        

//The Cons variant will need the size of an i32 plus the space to store the box’s pointer data. The Nil variant stores no values, 
//so it needs less space than the Cons variant. We now know that any List value will take up the size of an i32 plus the size of a box’s pointer data. 
enum List {
    Cons(i32, Box<List>),
    Nil(),
}

use crate::List::{Cons, Nil};

//MyBox type is a tuple struct
struct MyBox <T> (T);

impl<T> MyBox<T> {
    //new function to match box's
    fn new(x: T) -> MyBox<T>{
        MyBox(x)
    }
} 

//implement deref trait to enable dereferencing with * operator
//to implement a trait, we need to provide implementations for the trait’s required methods
//The Deref trait, provided by the standard library, requires us to implement
// one method named deref that borrows self and returns a reference to the inner data
impl<T> Deref for MyBox<T> {
    //this syntax defines an associated type for the Deref trait to use (associated types are slightly different way of declaring a generic param)
    type Target = T;
    fn deref(&self) -> &T{
        &self.0
    }
}
// Without the Deref trait, the compiler can only dereference & references. The deref method gives the compiler the ability 
// to take a value of any type that implements Deref and call the deref method to get a & reference that it knows how to dereference.

fn hello(name: &str) {
    println!("Hello, {}!", name);
}


//DROP trait
//you can specify that a particular bit of code be run whenever a value goes out of scope, and the compiler will insert this code automatically. 
//As a result, you don’t need to be careful about placing cleanup code everywhere in a program that an instance of a particular type is finished with—you still won’t leak resources
struct CustomSmartPointer {
    data : String,
}


impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("dropping out data {} ", self.data);
    }
}


//Reference counter. for multiple owners of same data. works only with single thread programs
use std::rc::Rc;
use crate::List2::{Cons as RcCons, Nil as RcNil};

enum List2 {
    Cons(i32, Rc<List2>),
    Nil,
}








fn main() {
    //Boxes provide only the indirection and heap allocation; they don’t have any other special capabilities, like other smart pointer types.
    let b = Box::new(5);
    println!("b = {}", b);

    //box can be dereferenced like normal pointers
    assert_eq!(5,*b);

    let number = 5;
    let c = MyBox::new(number);
    // When we entered *c in, behind the scenes Rust actually ran this code: *(y.deref())
    assert_eq!(5, *c);


    //At compile time, Rust needs to know how much space a type takes up. One type whose size can’t be known at compile time is a recursive type,
    // where a value can have as part of itself another value of the same type. 

 
    // let list = Cons(1,
    //     Box::new(Cons(2,
    //         Box::new(Cons(3,
    //             Box::new(Nil()))))));

    //We can call the hello function with a string slice as an argument, such as hello("Rust"); for example. 
    //Deref coercion makes it possible to call hello with a reference to a value of type MyBox<String>.
    //Deref coercion happens automatically when we pass a reference to a particular type’s value as an argument to a function
    //or method that doesn’t match the parameter type in the function or method definition.
    let m = MyBox::new(String::from("yo yo"));
    hello(&m);
    // /Here we’re calling the hello function with the argument &m, which is a reference to a MyBox<String> value. 
    //Because we implemented the Deref trait on MyBox<T> in Listing 15-10, Rust can turn &MyBox<String> into &String by calling deref.
    //If Rust didn’t implement deref coercion, we would have to write the code:
    hello(&(*m)[..]);
    //The (*m) dereferences the MyBox<String> into a String. 
    //Then the & and [..] take a string slice of the String that is equal to the whole string to match the signature of hello
    //The number of times that Deref::deref needs to be inserted is resolved at compile time, 
    //so there is no runtime penalty for taking advantage of deref coercion!

    // Rust does deref coercion when it finds types and trait implementations in three cases:
        // From &T to &U when T: Deref<Target=U>
        // From &mut T to &mut U when T: DerefMut<Target=U>
        // From &mut T to &U when T: Deref<Target=U>
    //Rust will also coerce a mutable reference to an immutable one. But the reverse is not possible: immutable references will never coerce to mutable references.

    let custompoint = CustomSmartPointer{data : String::from("str1")};
    println!("Custom smart pointer created!");
    //call the std::mem::drop function provided by the standard library if you want to force a value to be dropped before the end of its scope.
    let custompoint2 = CustomSmartPointer{data : String::from("str2")};
    //following allows us to drop data from our custompointer
    drop(custompoint2);
    println!("end of main");


    //rc example usage
    //The implementation of Rc::clone doesn’t make a deep copy of all the data like most types’ implementations of clone do. 
    //The call to Rc::clone only increments the reference count, which doesn’t take much time. Deep copies of data can take a lot of time.
    let a_list = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    println!("count after creating a = {}", Rc::strong_count(&a_list));
    let b_list = RcCons(3, Rc::clone(&a_list));
    println!("count after creating b = {}", Rc::strong_count(&a_list));
    {
        let c_list = RcCons(4, Rc::clone(&a_list));
        println!("count after creating c = {}", Rc::strong_count(&a_list));
    }    
    println!("count outscope = {}", Rc::strong_count(&a_list));
    //Via immutable references, Rc<T> allows you to share data between multiple parts of your program for reading only.

    

}



