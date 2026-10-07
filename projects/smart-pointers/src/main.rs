use smart_pointers::{box_demo, cycles, deref, drop_demo, rc, refcell};

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

    refcell::demo_refcell();

    cycles::demo_cycle();
}
