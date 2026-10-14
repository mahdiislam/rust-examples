// Slide 15.2 · A String is not 'static
//
// This program must NOT compile. rustc rejects it with:
//     error[E0597]: `name` does not live long enough
// `cargo test` checks that this is the only error code rustc reports.

struct Pet {
    name: &'static str,
}

fn main() {
    let name = format!("Fer{}", "ris");
    let p = Pet { name: &name };
    // error: `name` does not live long enough
}
