// Slide 19.12 · ...and it really moves
//
// This program must NOT compile. rustc rejects it with:
//     error[E0382]: use of moved value: `msg`
// `cargo test` checks that this is the only error code rustc reports.

fn run_once<F>(f: F)
where
    F: FnOnce() -> String,
{
    println!("{}", f());
}

fn main() {
    let msg = String::from("done");
    run_once(move || msg);
    run_once(move || msg); // error: use of moved value: `msg`
}
