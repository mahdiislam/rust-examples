// Slide 16.6 · Illegal: a heap-allocated Vec
//
// This program must NOT compile. rustc rejects it with:
//     error[E0597]: `v` does not live long enough
// `cargo test` checks that this is the only error code rustc reports.

fn rest(s: &[i32]) -> &[i32] {
    &s[1..]
}

fn main() {
    let tail = {
        let v = vec![7, 8, 9];
        rest(&v)
        // error: `v` does not live long enough
    };
    println!("tail = {:?}", tail);
}
