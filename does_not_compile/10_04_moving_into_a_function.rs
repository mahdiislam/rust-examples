// Slide 10.4 · Moving into a function
//
// This program must NOT compile. rustc rejects it with:
//     error[E0382]: use of moved value: `t`
// `cargo test` checks that this is the only error code rustc reports.

struct Temp {
    freezing: bool,
    degrees: i32,
}

fn show_temp(t: Temp) {
    println!("{}°", t.degrees);
}

fn main() {
    let t = Temp { freezing: false, degrees: 3 };
    show_temp(t); // `t` is moved
    show_temp(t); // error: use of moved value: `t`
}
