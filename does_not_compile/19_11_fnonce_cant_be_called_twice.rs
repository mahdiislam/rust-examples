// Slide 19.11 · FnOnce can't be called twice
//
// This program must NOT compile. rustc rejects it with:
//     error[E0382]: use of moved value: `f`
// `cargo test` checks that this is the only error code rustc reports.

fn run_once<F>(f: F)
where
    F: FnOnce() -> String,
{
    println!("{}", f());
    println!("{}", f()); // error: use of moved value: `f`
}

fn main() {
    run_once(|| String::from("done"));
}
