/*
Generics are abstract stand-ins for concrete types or other properties.
Similar to the way a function takes parameters with unknown values to run the same code on multiple concrete values, 
functions can take parameters of some generic type instead of a concrete type, like i32 or String, lke Option<T>, Vec<T>, HashMap<K, V>, and Result<T, E>. 

*/

//usage in structs, with same types
struct Point<T> {
    x: T,
    y: T,
}   

//with posibility of having different types
struct Point2<T, U> {
    x: T,
    y: U,
}

//method decleration, not impl<T> this outlines that this is a generic type, not a concrete type
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}   

// or we could declare only for one type, where we don't need impl<T>, but only impl
// This code means the type Point<f32> will have a method named distance_from_origin and other instances of Point<T> where T is not of type f32 will not have this method defined. 
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

//it is possible to mix types that are defined by the struct with some other types
//The method creates a new Point instance with the x value from the self Point (of type T) and the y value from the passed-in Point (of type W).
//Here, the generic parameters T and U are declared after impl, because they go with the struct definition.
impl<T, U> Point<T, U> {
    //The generic parameters V and W are declared after fn mixup, because they’re only relevant to the method.
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

//usage in enums
enum Option<T> {
    Some(T),
    None,
}

//with mltiple generic types
enum Result<T, E> {
    Ok(T),
    Err(E),
}

//to enable comparisons, the standard library has the std::cmp::PartialOrd trait that you can implement on types (see Appendix C for more on this trait)
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
