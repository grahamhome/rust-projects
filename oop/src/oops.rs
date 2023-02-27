
/// This is a sub-optimal implementation that hews closely to the OOP design pattern, but
/// doesn't take advantage of all of Rust's features.
pub struct Post {
    // We use Box to standardize the size of state regardless of its type.
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }

    pub fn content(&self) -> &str {
        // as_ref() gives us a Box<&dyn State>. We need it as a reference because we are borrowing
        // self so we can't take ownership of one of its members.
        self.state.as_ref().unwrap().content(self)
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<Self>) -> Box<dyn State>;

    fn reject(self: Box<Self>) -> Box<dyn State>;

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        // Posts have no content until approved, we provide a default implementation here.
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        // Approving a draft post has no effect on its state, it stays in the Draft state.
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // A post should stay in the pending review state if another review is requested
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // Requesting review on a published post has no effect
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        // Approving a published post has no effect
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle() {
        let mut post = Post::new();

        let content = "I had a potato for lunch today";

        post.add_text(content);

        assert_eq!(post.content(), "");

        post.approve();

        assert_eq!(post.content(), "");

        post.request_review();

        assert_eq!(post.content(), "");

        post.reject();

        post.request_review();

        post.approve();

        assert_eq!(post.content(), content);
    }
}