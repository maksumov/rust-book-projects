// ch 20.5: the derive crate (listings 20-36, 20-40..20-42). A
// proc-macro crate TYPE is required - procedural macros must live
// in their own crate (Listing 20-36's shape: a fn taking
// TokenStream in, TokenStream out, the attribute choosing the
// kind). The pipeline: syn parses the annotated code into a syntax
// tree (the 20-41 DeriveInput sketch: ident "Pancakes" among its
// fields), quote! templates the impl back into tokens. (The book
// nests this crate inside hello_macro/; flattened here for the
// workspace - the property it protects, hello_macro usable WITHOUT
// the derive, is preserved: no dependency between them.)
//
// The other two kinds, signatures only (one crate type serves all
// three) - an attribute macro taking TWO streams (attr + item):
// // #[proc_macro_attribute]
// // pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}
//
// and a function-like one, parsing its argument tokens:
// // #[proc_macro]
// // pub fn sql(input: TokenStream) -> TokenStream {}

use proc_macro::TokenStream;
use quote::quote;

// Listings 20-40/20-42: the outer fn is the same in nearly every
// derive crate - parse to a tree, delegate. unwrap (panic), not
// Result: the proc_macro API returns TokenStream only; the book
// notes production code would expect! with a precise message.
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate.
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation.
    impl_hello_macro(&ast)
}

// Listing 20-42: #name is quote!'s templating (the macro_rules! $x
// idea reborn); stringify! turns the expression into a string
// literal AT COMPILE TIME - no allocation, a mini-macro working
// inside our macro.
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let generated = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    generated.into()
}

// Beyond the book: an attribute macro, live (the first of the two
// signature-only kinds from the header above). TWO token streams
// arrive: the attribute's own tokens and the annotated item. A proc
// macro runs at COMPILE time - the println! below lands in the
// build log of the CONSUMER crate. An attribute macro MUST return
// the item or it vanishes: emitted here unchanged.
#[proc_macro_attribute]
pub fn logged(attr: TokenStream, item: TokenStream) -> TokenStream {
    let note = attr.to_string();
    println!("[logged] attr = `{note}`; item emitted unchanged");
    item
}
