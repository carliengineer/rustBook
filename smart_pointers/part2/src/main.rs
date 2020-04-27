/*
borrowing rules:
    At any given time, you can have either (but not both of) one mutable reference or any number of immutable references.
    References must always be valid.
With references and Box<T>, the borrowing rules’ invariants are enforced at compile time. With RefCell<T>, these invariants are enforced at runtime.
With references, if you break these rules, you’ll get a compiler error. With RefCell<T>, if you break these rules, your program will panic and exit.
The RefCell<T> type is useful when you’re sure your code follows the borrowing rules but the compiler is unable to understand and guarantee that.
RefCell<T> is only for use in single-threaded scenarios
Mutating the value inside an immutable value is the interior mutability pattern

Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:
Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owner.
    Box<T> allows immutable or mutable borrows checked at compile time; Single Owner
    Rc<T> allows only immutable borrows checked at compile time; Multi owners
    RefCell<T> allows immutable or mutable borrows checked at runtime. Single Owner
Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.
*/

   // With RefCell<T>, we use the borrow and borrow_mut methods, which are part of the safe API that belongs to RefCell<T>.
   // The borrow method returns the smart pointer type Ref<T>, and borrow_mut returns the smart pointer type RefMut<T>.
   //The RefCell<T> keeps track of how many Ref<T> and RefMut<T> smart pointers are currently active.
   //using RefCell<T> makes it possible to write a mock object that can modify itself to keep track of the messages it has seen
   // while you’re using it in a context where only immutable values are allowed.
   //You can use RefCell<T> despite its trade-offs to get more functionality than regular references provide.

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}
//We want a Node to own its children, and we want to share that ownership with variables so we can access each Node in the tree directly. So Vec<Rc<Node>>
//We also want to modify which nodes are children of another node, so we have a RefCell<T> in children
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}



fn main() {
    //A common way to use RefCell<T> is in combination with Rc<T>. Recall that Rc<T> lets you have multiple owners of some data, but it only gives immutable access to that data.
    //If you have an Rc<T> that holds a RefCell<T>, you can get a value that can have multiple owners and that you can mutate!
    // let value = Rc::new(RefCell::new(5));

    // let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    // let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    // let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    // *value.borrow_mut() += 10;

        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // Uncomment the next line to see that we have a cycle;
        // it will overflow the stack
        // println!("a next item = {:?}", a.tail());
    //Creating reference cycles is not easily done, but it’s not impossible either. If you have RefCell<T> values that contain Rc<T> values or similar nested combinations of types
    // with interior mutability and reference counting, you must ensure that you don’t create cycles; you can’t rely on Rust to catch them.

    //You can also create a weak reference to the value within an Rc<T> instance by calling Rc::downgrade and passing a reference to the Rc<T>. When you call Rc::downgrade, you get a smart pointer of type Weak<T>.
    //Instead of increasing the strong_count in the Rc<T> instance by 1, calling Rc::downgrade increases the weak_count by 1.

    //Strong references are how you can share ownership of an Rc<T> instance. Weak references don’t express an ownership relationship.
    //They won’t cause a reference cycle because any cycle involving some weak references will be broken once the strong reference count of values involved is 0.

    // let leaf = Rc::new(Node {
    //     value: 3,
    //     parent: RefCell::new(Weak::new()),
    //     children: RefCell::new(vec![]),
    // });
    //
    // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    //
    //
    // let branch = Rc::new(Node {
    //     value: 5,
    //     parent: RefCell::new(Weak::new()),
    //     children: RefCell::new(vec![Rc::clone(&leaf)]),
    // });
    // *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );



}
