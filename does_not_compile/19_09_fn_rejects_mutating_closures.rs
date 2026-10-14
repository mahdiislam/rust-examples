// Slide 19.9 · Fn rejects mutating closures
//
// This program must NOT compile. rustc rejects it with:
//     error[E0594]: cannot assign to `calls`, as it is a captured variable in a `Fn` closure
// `cargo test` checks that this is the only error code rustc reports.

fn apply<F>(f: F)
where
    F: Fn(i32) -> i32,
{
    println!("{}", f(f(3)));
}

fn main() {
    let mut calls = 0;
    apply(|x| {
        calls += 1;
        // error: cannot assign to `calls`, as it is a
        //        captured variable in a `Fn` closure
        x * calls
    });
}
