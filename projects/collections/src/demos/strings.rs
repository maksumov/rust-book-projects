// `String::from(...)` and `.to_string()` are equivalent ways to
// build from a literal; `String::new()` gives an empty string.
// Note: unlike `Vec::new()`, no type annotation is needed here --
// `String::new()` returns the concrete `String`, not a generic.
pub fn strings_creation_demo() {
    println!("\n*** Strings creation demo ***");

    let empty = String::new();

    println!("Empty string initialized via String::new(): {empty:?}");

    let data = "first initial contents";

    let string_from_literal = data.to_string();

    // The method also works on a literal directly:
    let string_to_string_direct = "second initial contents".to_string();

    println!(
        "New strings created using .to_string() method: {string_from_literal:?} and {string_to_string_direct:?}"
    );

    let string_from_associated_fn = String::from("third initial contents");

    println!(
        "New string created using String::from associated function: {string_from_associated_fn:?}"
    );

    let different_hellos = [
        String::from("السلام عليكم"),
        String::from("Dobrý den"),
        String::from("Hello"),
        String::from("שלום"),
        String::from("नमस्ते"),
        String::from("こんにちは"),
        String::from("안녕하세요"),
        String::from("你好"),
        String::from("Olá"),
        String::from("Здравствуйте"),
        String::from("Hola"),
    ];

    println!("Hellos in different languages: {different_hellos:#?}");
}
