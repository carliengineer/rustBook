#[derive(PartialEq, Debug)]
struct Shoe{
    size : u32,
    style : String,
}

//The shoes_in_my_size function takes ownership of a vector of shoes and a shoe size
fn shoes_in_my_size(shoes : Vec<Shoe>, shoe_size : u32) -> Vec<Shoe>{
    //we call into_iter to create an iterator that takes ownership of the 
    //we call filter to adapt that iterator into a new iterator that only contains elements for which the closure returns true
    //The closure captures the shoe_size parameter from the environment and compares the value with each shoe’s size, keeping only shoes of the size specified. 
    //Finally, calling collect gathers the values returned by the adapted iterator into a vector that’s returned by the function.
    shoes.into_iter()
    .filter(|s| s.size == shoe_size)
    .collect()
}


#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}

//Let's create our own iterator that counts only from 1 to 5
//You can also create iterators that do anything you want by implementing the Iterator trait on your own types.
// As previously mentioned, the only method you’re required to provide a definition for is the next method.
struct Counter {
    count : u32,
}

impl Counter {
    fn new() -> Counter {
        //The count field is private because we want the implementation of Counter to manage its value.
        Counter { count: 0 }
    }

}

//implement the Iterator trait for our Counter type by defining the body of the next method to specify what we want to happen when this iterator is used
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
    //assert_eq!(counter.next(), Some(1));

}


#[test]
fn name() {
    let mut vec = Vec::new();

    vec = Counter::new().zip(Counter::new().skip(1))
                        .map(|(a,b)| a*b)
                        .filter(|x| x % 3 == 0)
                        .collect();

    for iter in vec.iter() {
        println!("asdasd: {}",iter );
    } 
    
    assert_eq!(vec[0], 6);
    assert_eq!(vec[1], 12);
    assert_eq!(vec.len(), 2);
    
}