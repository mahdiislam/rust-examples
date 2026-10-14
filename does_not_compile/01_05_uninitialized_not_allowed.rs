// Slide 01.5 · Uninitialized: not allowed
//
// This program must NOT compile. rustc rejects it with:
//     error[E0381]: used binding `total` is possibly-uninitialized
// (the slide says "isn't initialized"; current rustc words it this way)
// `cargo test` checks that this is the only error code rustc reports.

fn report(n: i64) {
    println!("report({n})");
}

fn main() {
    let total;
    report(total); // error: used binding `total` isn't initialized
    total = 3;
}
