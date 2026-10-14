// Slide 08.3 · Immutable by default: fields
//
// This program must NOT compile. rustc rejects it with:
//     error[E0594]: cannot assign to `t.degrees`, as `t` is not declared as mutable
// `cargo test` checks that this is the only error code rustc reports.

struct Temp {
    freezing: bool,
    degrees: i32,
}

fn main() {
    let t = Temp { freezing: false, degrees: 12 };
    t.degrees = 14; // error: cannot assign to `t.degrees`,
                    // as `t` is not declared as mutable
}
