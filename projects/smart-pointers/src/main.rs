mod box_demo;
mod deref;
mod drop_demo;
mod rc;

fn main() {
    box_demo::demo_box();
    box_demo::demo_cons();

    deref::demo_deref_operator();
    deref::demo_deref_on_box();
    deref::demo_my_box();
    deref::demo_deref_coercion();

    drop_demo::demo_drop_example();

    rc::demo_list();
    rc::demo_strong_count();
}
