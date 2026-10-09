mod futures;

// usage: cargo run -- <url> <url2>
// main stays sync by design: each demo bridges to async via its own
// block_on -- the E0752 story belongs to the demos, not to main.
fn main() {
    let args: Vec<String> = std::env::args().collect();

    futures::demo_page_title(&args[1]);
    futures::demo_race(&args[1], &args[2]);
}
