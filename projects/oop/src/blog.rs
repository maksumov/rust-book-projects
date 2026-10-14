// ch 18.3: an object-oriented design pattern ы- the state pattern, the
// OO version (listings 18-11..18-18). Behavior depends on the state;
// transitions live INSIDE the state objects (self: Box<Self> consumes
// the old state); Post knows nothing about the rules. Costs: states
// couple to each other, and some duplication - see the type-state
// rewrite in blog_typestate.rs.

// Listing 18-11: the desired behavior (the book's asserts), plus a
// beyond-the-book probe: approving a DRAFT does nothing.
pub fn demo_workflow() {
    println!("\n*** demo of the state pattern: a post's workflow (OO version) ***");

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
    println!("draft content: {:?}", post.content());

    post.approve(); // no-op here: Draft::approve returns self
    assert_eq!("", post.content()); // still a draft - approving before a review does nothing

    post.request_review();
    assert_eq!("", post.content());
    println!("pending review content: {:?}", post.content());

    post.approve(); // the real one: PendingReview -> Published
    assert_eq!("I ate a salad for lunch today", post.content());
    println!("published content: {:?}", post.content());
}

// Listing 18-12: state is PRIVATE - there is no way to create a Post
// in any state other than Draft (Post::new is the only door in).
pub struct Post {
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

    // Listing 18-13: add_text never touches the state - state-INDEPENDENT
    // behavior lives directly on Post; only state-dependent behavior
    // delegates through the trait.
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // Listings 18-14 -> 18-17: content EVOLVED. The first version was the
    // placeholder - always "", "the simplest thing that will fulfill
    // this requirement" while only the draft-era asserts had to pass.
    // Now it delegates to the state: the as_ref dance because state
    // cannot move out of &self; the unwrap is the ch 9 "more information
    // than the compiler" case - Post's own methods guarantee Some.
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    // Listing 18-15: why Option and the take dance? The transition must
    // CONSUME the old state (self: Box<Self>), so we need OWNERSHIP of
    // the Box - but a field cannot be moved out from behind &mut self:
    //
    // // self.state = self.state.request_review();  // error[E0507]:
    // //                                            // cannot move out of
    // //                                            // `self.state`
    //
    // take() is the escape: it moves the Some out (leaving None -
    // Rust forbids unpopulated fields, so None stands in for the
    // instant when no state exists), the transition consumes the old
    // state, the result moves back in. request_review and approve
    // share this shape.
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    // Listing 18-16: the shared take dance (request_review above).
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

impl Default for Post {
    // clippy's new_without_default again (the AveragedCollection story),
    // manual this time: the dyn State field cannot derive.
    fn default() -> Post {
        Post::new()
    }
}

trait State {
    // Listing 18-15: self: Box<Self> - the method is valid only when
    // called on a Box, and takes OWNERSHIP of it: the old state value
    // dies as it transforms into the new one. This is what makes
    // transitions impossible to bypass.
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    // Listing 18-16: the same Box<Self> ownership story.
    fn approve(self: Box<Self>) -> Box<dyn State>;

    // Listing 18-18: the default returns "" - Draft and PendingReview
    // inherit it; Published overrides with the real content. Virtual
    // methods, the Rust way.
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

// Listing 18-15/18-16: Draft - request_review -> PendingReview;
// approve -> stays (approving a draft has no effect).
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

// Listing 18-15/18-16: PendingReview - request_review -> stays;
// approve -> Published.
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

// Listing 18-16: Published - the terminal state:
// request_review and approve both -> stays (nothing leaves Published).
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // Listing 18-18 (override):
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
