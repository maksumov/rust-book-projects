// Listing 9-5: different actions for different failure reasons.
// The Err arm matches on error.kind(): NotFound -> create the file
// and return the new handle (File::create can fail too -- hence
// the third nested match); anything else still panics. Side effect:
// run without hello.txt and this demo CREATES it -- delete the file
// to watch the NotFound branch again. The book follows up with a
// closure-based unwrap_or_else alternative (chapter 13 preview).

pub fn demo() {
    use std::fs::File;
    use std::io::ErrorKind;

    let filename = "hello.txt";
    let greeting_file_result = File::open(filename);

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file {filename:?}: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file {filename:?}: {error:?}");
            }
        },
    };

    dbg!(greeting_file);
}
