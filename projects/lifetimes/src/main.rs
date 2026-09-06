// Chapter 10.3 demos, one module per topic (see the module files
// for the listings and commented compiler errors):
mod dangling;
mod excerpt;
mod longest;

fn main() {
    dangling::demo();
    longest::demo();
    longest::relationships_demo();
    excerpt::demo();
}
