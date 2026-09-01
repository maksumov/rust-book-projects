// The demos open "hello.txt" in the CURRENT directory: create it
// (`touch hello.txt`) to see the Ok branch, delete it for the
// Err/panic paths. hello.txt is an experiment artifact and is not
// committed.

mod open_match;

fn main() {
    open_match::demo();
}
