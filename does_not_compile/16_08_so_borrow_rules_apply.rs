// Slide 16.8 · ...so borrow rules apply
//
// This program must NOT compile. rustc rejects it with:
//     error[E0597]: `file` does not live long enough
// `cargo test` checks that this is the only error code rustc reports.

fn extension(file: &str) -> Option<&str> {
    // no new string: returns a slice of `file`
    file.rsplit_once('.').map(|(_, ext)| ext)
}

fn main() {
    let ext = {
        let file = String::from("notes.final.md");
        extension(&file).unwrap_or("")
        // error: `file` does not live long enough
    };
    println!("extension: {:?}", ext);
}
