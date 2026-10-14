use oop::encapsulation;

mod select_box;

fn main() {
    encapsulation::demo_averaged_collection();

    // From OUTSIDE the module the type cannot be assembled by hand:
    //
    // // let c = AveragedCollection {                       // error[E0603]:
    // //     list: vec![1, 2, 3],                           // field `list` of
    // //     average: 2.0,                                  // struct `...` is
    // // };                                                 // private
    //
    // The whole point of listing 18-1: only the public API can touch
    // the data -- so `average` can never fall out of sync with `list`.

    select_box::demo_screen();
}
