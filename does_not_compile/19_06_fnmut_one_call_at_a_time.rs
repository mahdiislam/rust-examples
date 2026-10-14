// Slide 19.6 · FnMut: one call at a time
//
// This program must NOT compile. rustc rejects it with:
//     error[E0499]: cannot borrow `f` as mutable more than once at a time
// `cargo test` checks that this is the only error code rustc reports.

fn apply<F>(mut f: F)
where
    F: FnMut(i32) -> i32,
{
    println!("{}", f(f(3)));
    // error: cannot borrow `f` as mutable more than once at a time
}

fn main() {
    apply(|x| x + 1);
}
