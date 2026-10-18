// Enforce the SAFETY comment convention mechanically: every unsafe
// block must carry a `// SAFETY:` justification.
#![warn(clippy::undocumented_unsafe_blocks)]

mod traits;
mod unsafe_rust;

fn main() {
    unsafe_rust::demo_raw_pointers_deref();
    unsafe_rust::demo_unsafe_fn();
    unsafe_rust::demo_ffi();
    unsafe_rust::demo_ffi_safe();
    unsafe_rust::demo_static();
    unsafe_rust::demo_unsafe_trait();

    traits::demo_associated_types();
}
