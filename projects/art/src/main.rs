// Before the re-exports in lib.rs, imports needed the full module path:
// use art::kinds::PrimaryColor;
// use art::utils::mix;

use art::{PrimaryColor, SecondaryColor, mix};

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    let result = mix(red, yellow).unwrap();

    assert_eq!(result, SecondaryColor::Orange);
}
