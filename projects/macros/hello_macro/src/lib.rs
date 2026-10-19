// ch 20.5: procedural macros - the hello_macro case (listings
// 20-36..20-42). A lib crate with a trait (20-38) whose impl users
// write by hand (20-39) - until the derive crate generates it
// (20-40..20-42, next commit). Why a macro at all: Rust has no
// reflection - a default trait method cannot learn the type's NAME
// at runtime; only compile-time generation can.

// Listing 20-38: the trait. The whole crate so far - a user could
// implement it manually (pancakes does, for now).
pub trait HelloMacro {
    fn hello_macro();
}
