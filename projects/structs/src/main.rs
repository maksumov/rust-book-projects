// Structs holding references require lifetime annotations
// to ensure the data they point to stays valid (see chapter 10).
// This does not compile: `missing lifetime specifier` error.
struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}
