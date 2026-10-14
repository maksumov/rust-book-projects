# Chapter 18: The State Pattern - Exercises

1. From the book's "Why Not An Enum?": rebuild the post workflow a
   THIRD way - `Post { state: State, content: String }` with
   `enum State { Draft, PendingReview, Published }` and match arms in
   every method. Compare with the two implementations in the repo:
   where do the rules live now? How many places must change to add a
   `Scheduled` state?
2. From the book: add a `reject` method (PendingReview -> Draft).
3. From the book: require two approvals before publishing.
4. From the book: allow `add_text` only in Draft. Hint: the state
   object decides what MAY change about the content, without being
   responsible for modifying the Post.

Source: https://doc.rust-lang.org/stable/book/ch18-03-oo-design-patterns.html

From the book:

> That's certainly a possible solution; try it and compare the end
> results to see which you prefer! One disadvantage of using an enum
> is that every place that checks the value of the enum will need a
> `match` expression or similar to handle every possible variant.

> Add a `reject` method that changes the post's state from
> `PendingReview` back to `Draft`.

> Require two calls to `approve` before the state can be changed to
> `Published`.

> Allow users to add text content only when a post is in the `Draft`
> state. Hint: have the state object responsible for what might change
> about the content but not responsible for modifying the `Post`.
