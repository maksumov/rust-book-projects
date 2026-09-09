mod capturing_references;
mod function_vs_closure;
mod shirt_company;
mod type_annotations;

fn main() {
    shirt_company::demo();

    type_annotations::demo_first();
    type_annotations::demo_second();
    type_annotations::demo_third();

    // beyond the book
    function_vs_closure::demo();

    capturing_references::demo_immutable();
    capturing_references::demo_mutable();
    capturing_references::demo_moving_ownership();
}
