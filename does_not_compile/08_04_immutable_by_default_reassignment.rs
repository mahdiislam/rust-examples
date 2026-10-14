// Slide 08.4 · Immutable by default: reassignment
//
// This program must NOT compile. rustc rejects it with:
//     error[E0384]: cannot assign twice to immutable variable `t`
// `cargo test` checks that this is the only error code rustc reports.

struct Temp {
    freezing: bool,
    degrees: i32,
}

fn main() {
    let t = Temp { freezing: false, degrees: 12 };
    t = Temp { freezing: true, degrees: -1 };
    // error: cannot assign twice to immutable variable `t`
}
