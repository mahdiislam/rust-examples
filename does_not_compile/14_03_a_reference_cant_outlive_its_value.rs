// Slide 14.3 · A reference can't outlive its value
//
// This program must NOT compile. rustc rejects it with:
//     error[E0597]: `n` does not live long enough
// `cargo test` checks that this is the only error code rustc reports.

fn main() {
    let r = {
        let n = 5;
        &n
    };
    println!("r = {}", r);
    // error: `n` does not live long enough
}
