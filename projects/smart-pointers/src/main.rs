mod box_demo;
mod deref;

fn main() {
    box_demo::demo_box();
    box_demo::demo_cons();

    deref::demo_deref_operator();
}
