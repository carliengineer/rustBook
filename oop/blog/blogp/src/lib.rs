#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}



impl Post {
    pub fn new() {
        //why option: we call the take method to take the Some value out of the state field and leave a None in its place, 
        //because Rust doesn’t let us have unpopulated fields in structs. 
        //This lets us move the state value out of Post rather than borrowing it.
        state : Option<Box<dyn State>>,
        content : String,
    }

    pub fn content(&self) -> &str{
        self.state.as_ref().unwrap().content(self)  
    }

    pub fn add_text(&mut self, text :&str) {
        self.content.push_str(text);
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

// The State trait defines the behavior shared by different post states
trait State {
    //we have self: Box<Self>. This syntax means the method is only valid when called on a Box holding the type. 
    //This syntax takes ownership of Box<Self>, invalidating the old state so the state value of the Post can transform into a new state.
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    //We add a default implementation for the content method that returns an empty string slice. 
    //That means we don’t need to implement content on the Draft and PendingReview structs
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }

}

struct Draft {

}

impl State for Draft {
    fn request_review(self: <Box<Self>>) -> Box<dyn State>{
        Box::new(PendingReview{})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

}

struct PendingReview {

}

impl State for PendingReview {
    fn request_review(self: <Box<Self>>) -> Box<dyn State>{
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

}

struct Published {

}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}