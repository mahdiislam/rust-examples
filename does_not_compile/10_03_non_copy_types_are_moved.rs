// Slide 10.3 · Non-Copy types are moved
//
// This program must NOT compile. rustc rejects it with:
//     error[E0382]: use of moved value: `t`
// `cargo test` checks that this is the only error code rustc reports.

struct Temp {
    freezing: bool,
    degrees: i32,
}

fn main() {
    let t = Temp { freezing: false, degrees: 3 };
    let u = t; // `t` is moved into `u`
    let v = t; // error: use of moved value: `t`
}
