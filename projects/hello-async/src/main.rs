mod futures;

// usage: cargo run -- <url> [url2]   (17-5's race will take the second)
// main stays sync by design: each demo bridges to async via its own
// block_on -- the E0752 story belongs to the demos, not to main.
fn main() {
    let args: Vec<String> = std::env::args().collect();

    futures::demo_page_title(&args[1]);
}
