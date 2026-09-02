// Listing 9-11: `?` also works on Option -- None returns early,
// Some unwraps. The enclosing function must return the SAME Option;
// Result and Option can't be mixed without conversion (ok / ok_or).

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

pub fn demo() {
    println!("\n*** question_on_option demo: `?` on Option ***");

    println!(
        "regular text:   {:?}",
        last_char_of_first_line("Hello, world\nHow are you?")
    ); // Some('d')
    println!("empty string:   {:?}", last_char_of_first_line("")); // None
    println!("blank 1st line: {:?}", last_char_of_first_line("\nhi")); // None
}
