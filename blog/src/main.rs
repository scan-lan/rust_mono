struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn add_content(&mut self, content: &str) {
        self.content.push_str(content);
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
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft;

impl State for Draft {
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

struct PendingReview;

impl State for PendingReview {
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published)
    }

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct Published;

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

fn main() {
    let mut p = Post::new();
    assert_eq!(p.content(), "".to_string());

    p.add_content("content");
    assert_eq!(p.content(), "".to_string());
}

#[cfg(test)]
mod test {
    use crate::Post;

    #[test]
    fn post_content_inaccessible() {
        let mut p = Post::new();
        assert_eq!(p.content(), "".to_string());

        p.add_content("content");
        assert_eq!(p.content(), "".to_string());
    }

    #[test]
    fn content_accessible_after_approval() {
        let mut p = Post::new();
        p.add_content("content");
        p.request_review();
        p.approve();
        assert_eq!(p.content(), "content");
    }
}
