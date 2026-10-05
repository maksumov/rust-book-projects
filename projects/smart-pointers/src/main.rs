mod box_demo;
mod deref;

fn main() {
    box_demo::demo_box();
    box_demo::demo_cons();

    deref::demo_deref_operator();
    deref::demo_deref_on_box();
    deref::demo_my_box();
    deref::demo_deref_coercion();
}
