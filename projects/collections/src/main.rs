// Chapter 8 demos, one file per collection under demos/:
// `mod demos;` loads demos.rs, its children live in demos/.
mod demos;

use crate::demos::{hashmaps, strings, vectors};

fn main() {
    vectors::vectors_creation_demo();
    vectors::vectors_updating_demo();
    vectors::vectors_reading_elements_demo();
    vectors::vectors_borrow_conflict_demo();
    vectors::vectors_iteration_demo();
    vectors::vectors_multiple_types_demo();

    strings::strings_creation_demo();
    strings::strings_appending_demo();
    strings::strings_concatenation_demo();
    strings::strings_indexing_into_demo();
    strings::strings_internal_representation_demo();
    strings::strings_views_demo();
    strings::strings_slicing_demo();
    strings::strings_iteration_demo();

    hashmaps::hashmaps_creation_demo();
    hashmaps::hashmaps_creation_via_collect_demo();
    hashmaps::hashmaps_accessing_values_demo();
    hashmaps::hashmaps_iteration_demo();
    hashmaps::hashmaps_managing_ownership_demo();
    hashmaps::hashmaps_overwriting_value_demo();
}
