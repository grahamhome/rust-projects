// A better implementation in which the states are encoded in the struct types.
// With this design, any illegal action will result in a compiler error.

pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {

    /// We can't just create a Post, we have to get a DraftPost and move it though the approval process
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new()
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // No content() method = no way to get the content of a DraftPost

    /// Takes ownership of self, "consuming" the DraftPost so it can no longer be used.
    /// This is why we are able to return a new PendingReviewPost that owns the content formerly owned by the
    /// DraftPost.
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    /// Takes ownership of self, "consuming" the PendingReviewPost so it can no longer be used.
    /// This is why we are able to return a new Post that owns the content formerly owned by the
    /// PendingReviewPost.
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle_again() {
        let mut post = Post::new();

        let content = "I ate a potato for lunch today";

        post.add_text(content);

        let post = post.request_review();

        let post = post.reject();

        let post = post.request_review();

        let post = post.approve();

        assert_eq!(post.content(), content);
    }
}