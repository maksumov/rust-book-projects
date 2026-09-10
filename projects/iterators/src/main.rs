mod iteration_forms;
mod iterator_adapters;
mod laziness;

fn main() {
    laziness::demo();

    // beyond the book
    iteration_forms::demo();

    iterator_adapters::demo();

    // beyond the book
    iterator_adapters::demo_infinite();
}
