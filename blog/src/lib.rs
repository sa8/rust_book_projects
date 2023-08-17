pub struct Post {
    //state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
       // Post { state: Some(Box::new(Draft{})), content: String::new(), }
       DraftPost {
        content: String::new(),
       }
    }

    // pub fn add_text(&mut self, text:&str) {
    //     self.content.push_str(text);
    // }

    pub fn content(&self) -> &str {
        //self.state.as_ref().unwrap().content(self)
        &self.content
    }
    
    // pub fn request_review(&mut self) {
    //     if let Some(s) = self.state.take() {
    //         self.state = Some(s.request_review())
    //     }
    // }

    // pub fn approve(&mut self) {
    //     if let Some(s) = self.state.take() {
    //         self.state = Some(s.approve())
    //     }
    // }
}

// trait State {
//     fn request_review(self: Box<Self>) -> Box<dyn State>;
//     fn approve(self: Box<Self>) -> Box<dyn State>;
//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         ""
//     }
// }

pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {  content: self.content,}
    }
    // fn request_review(self: Box<Self>) -> Box<dyn State> {
    //     Box::new(PendingReview {})
    // }

    // fn approve(self: Box<Self>) -> Box<dyn State> {
    //     self
    // }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    // fn request_review(self: Box<Self>) -> Box<dyn State> {
    //     self
    // }
    pub fn approve(self) -> Post {
       // Box::new(Published {})
       Post { content:self.content, }
    }
    
}

// struct Published {}

// impl State for Published {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }

//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }

//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         &post.content
//     }
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
