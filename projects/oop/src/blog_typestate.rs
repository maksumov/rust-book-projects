// ch 18.3: the same workflow, states as TYPES (listings 18-19..18-21).
// DraftPost -> PendingReviewPost -> Post: transitions consume self and
// return the NEXT type. content() exists ONLY on the published Post -
// using a draft's content is a COMPILE error, not a runtime "".
// The trade: Post no longer encapsulates the transitions (main
// shadows its way through them).

pub fn demo_workflow() {
    println!("\n*** demo of the state pattern: the type-state version ***");

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    // The draft-era asserts are not merely unnecessary here - they are
    // IMPOSSIBLE: DraftPost has no content method, E0599. That is the
    // entire point of this design: invalid states do not compile.
    // // assert_eq!("", post.content());

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
    println!("published content: {:?}", post.content());
}

// Listing 18-19: Post = the published type; Post::new() returns a
// DraftPost - no way to assemble a Post by hand (content is private).
pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl Post {
    // The deliberate API of this design: asking for a Post gives you a
    // DraftPost - the only road to a published Post goes through the
    // workflow. Deviates from the lint's convention on purpose.
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
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

    // Listing 18-20: the transitions - request_review and approve take
    // OWNERSHIP (self), consuming the old value and returning the next
    // type: no lingering DraftPost can exist after the move. The 18-21
    // demo below shadows its way through the workflow.
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
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
