// Listings 9-3 + 9-4: `File::open` returns Result<File, io::Error>:
// the generic T/E are filled in by its signature (handle vs io
// error). Handled with a match -- the same tool as for Option in
// chapter 6; Ok/Err need no `Result::` prefix (both are in the
// prelude). Panics when hello.txt is missing: create/delete the
// file to observe both branches.

pub fn demo() {
    use std::fs::File;

    let filename = "hello.txt";
    let greeting_file_result = File::open(filename);

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file {filename:?}: {error:?}"),
    };

    dbg!(greeting_file);
}
