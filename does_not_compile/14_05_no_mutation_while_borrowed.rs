// Slide 14.5 · No mutation while borrowed
//
// This program must NOT compile. rustc rejects it with:
//     error[E0506]: cannot assign to `n` because it is borrowed
// `cargo test` checks that this is the only error code rustc reports.

fn main() {
    let mut n = 5;
    let r = &n;
    n = 6;
    println!("r = {}", r);
    // error: cannot assign to `n` because it is borrowed
}
