mod places;
mod refutability;
mod syntax;

fn main() {
    places::demo_background();

    refutability::demo_let_else();

    syntax::demo_shadowing_traps();
    syntax::demo_destructuring();
    syntax::demo_ignoring();
    syntax::demo_at_bindings();
}
