// Slide 14.6 · Shared XOR mutable
//
// This program must NOT compile. rustc rejects it with:
//     error[E0502]: cannot borrow `n` as mutable because it is also borrowed as immutable
// `cargo test` checks that this is the only error code rustc reports.

fn main() {
    let mut n = 5;
    let shared = &n;
    let unique = &mut n;
    // error: cannot borrow `n` as mutable because
    //        it is also borrowed as immutable
    println!("{}", shared);
}
