
//Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.
pub trait Summary {
    fn summarize(&self) -> String; //;
    //defining a default implmentation, string in this case, for a trait, by putting it in addtion to method's signature
    //{ 
      //  String::from("(Read more...)")
    //}
}

//new type and 
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("News: {}, by {} ({})", self.headline, self.author, self.location)
    }
}

//new type and implementtion of trait
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

//difference is "for" keyword
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("Tweet: {}: {}", self.username, self.content)
    }
}

// Instead of a concrete type for the item parameter, we specify the impl keyword and the trait name. 
//This parameter accepts any type that implements the specified trait.
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// trait bound
// we place trait bounds with the decleration of the generic type parameter after a colon and inside angle brackets.
/*
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
*/

//when calling from outside, use crate_name::trait_name 
//trait should be public in order for other crate to implement it.
// orphan rule: But we can’t implement external traits on external types. so in your local crate, you can't use Vec<T> with Display together 
// as both are defined in the standard library and aren't local to our current crate. 

fn main() {
    //A type’s behavior consists of the methods we can call on that type. Different types share the same behavior if we can call the same methods on all of those types. 

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    
    println!("1 new tweet: {}", tweet.summarize());

    println!("{:?}", notify(tweet));


}
